use crate::build_modules::builder_generator::BuilderGenerator;
use crate::build_modules::items::{Item, ItemTrait, ModuleItems, SourceFile};
use crate::build_modules::utils::{create_use, create_use_as_glob, generate_source_code, path::Path};
use itertools::Itertools;
use openapiv3::v2::{Operation, Parameter, ReferenceOrSchema};
use pipewire_common::debug;
use serde_yaml;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::process::{Command, Stdio};
use std::rc::Rc;
use syn::{Field, Fields, FieldsNamed, Token};
use syn::punctuated::Punctuated;
use tempfile::NamedTempFile;
use crate::build_modules::api_generator::ApiGenerator;
use crate::build_modules::configuration_transformer::ConfigurationTransformer;

mod build_modules;

fn main() {    
    let tool_path = std::path::Path::new("resources")
        .join("tools")
        .join("openapi-generator");
    let output_path = output_directory();
    // let output_path = tool_path.join("output");
    install_yarn_tool(&tool_path);
    let specification_file = transform_openapi_document(api_version());
    let sync_client_cargo_path = generate_client(
        &tool_path,
        specification_file,
        "container-api",
        client_version(),
        false
    );
    let mut model_source_file = merge_model_source_files(&sync_client_cargo_path);
    generate_source_code(&mut model_source_file, &output_path, "models.rs");
    let mut api_source_file = merge_api_source_files(&sync_client_cargo_path);
    generate_source_code(&mut api_source_file, &output_path, "api-sync.rs");
    let mut params_source_file = extract_params_structs(&mut api_source_file);
    generate_source_code(&mut params_source_file, &output_path, "params.rs");
    let mut builders_source_file = create_params_builders(&model_source_file, &params_source_file);
    generate_source_code(&mut builders_source_file, &output_path, "builders.rs");
}

fn output_directory() -> std::path::PathBuf {
    let output_direction = std::env::var("OUT_DIR")
        .expect("OUT_DIR not set");
    std::path::Path::new(&output_direction).to_path_buf()
}

fn client_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn api_version() -> &'static str {
    #[cfg(feature = "v1_47_0")]
    "v1.47"
}

fn command(
    command: &str,
    arguments: Option<&[&str]>,
    working_dir: &std::path::Path,
) {
    let mut command = Command::new(command);
    if arguments.is_some() {
        command.args(arguments.unwrap());
    }
    let mut process = command
        .current_dir(working_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let stderr = process.stderr.take().expect("Failed to capture stderr");
    let stderr_reader = BufReader::new(stderr);
    for line in stderr_reader.lines() {
        match line {
            Ok(output) => debug!("{}", output),
            Err(e) => debug!("Error reading output: {}", e),
        }
    }
    process.wait().expect("Failed to wait on child");
}

fn install_yarn_tool(tool_path: &std::path::Path) {
    command(
        "yarn",
        None,
        tool_path
    )
}

fn transform_openapi_document(
    api_version: &str,
) -> NamedTempFile
{
    fn transform_parameter(parameter: &mut Parameter) -> bool {
        if parameter.schema.is_none() {
            return false;
        }
        let parameter_schema = parameter.schema.as_mut().unwrap();
        let parameter_schema = match parameter_schema {
            ReferenceOrSchema::Reference { .. } => return false,
            ReferenceOrSchema::Item(value) => value
        };
        if parameter_schema.schema_type.is_none() || parameter_schema.format.is_none() {
            return false;
        }
        let schema_type = parameter_schema.schema_type.as_ref().unwrap();
        let schema_format = parameter_schema.format.as_ref().unwrap();
        if schema_type != "string" || schema_format != "binary" || parameter.required.is_some() {
            return false;
        }
        parameter.required = Some(true);
        debug!(
            "Parameter transformed: {}+{}: required: {}",
            schema_type,
            schema_format,
            true,
        );
        true
    }
    fn transform_operation(operation: &mut Option<Operation>) {
        if operation.is_none() {
            return;
        }
        let operation = operation.as_mut().unwrap();
        if operation.parameters.is_none() {
            return;
        }
        for parameter in operation.parameters.as_mut().unwrap().iter_mut() {
            let result = transform_parameter(parameter);
            if result {
                debug!("in operation {}", operation.operation_id.as_ref().unwrap());
            }
        }
    }
    let document_path = std::path::Path::new("resources")
        .join("specs")
        .join(format!("{}.yaml", api_version))
        .to_str()
        .unwrap()
        .to_string();
    let document_content = fs::read_to_string(document_path).unwrap();
    let mut spec: openapiv3::v2::OpenAPI = serde_yaml::from_str(document_content.as_str()).expect("Could not structure OpenAPI file");
    for (_, path) in spec.paths.iter_mut() {
        transform_operation(&mut path.get);
        transform_operation(&mut path.post);
        transform_operation(&mut path.put);
        transform_operation(&mut path.patch);
        transform_operation(&mut path.delete);
        transform_operation(&mut path.head);
        transform_operation(&mut path.options);
    }
    let document_content = serde_yaml::to_string(&spec).unwrap();
    let mut output_file = NamedTempFile::new().unwrap();
    output_file.write_all(document_content.as_bytes()).unwrap();
    output_file.seek(SeekFrom::Start(0)).unwrap();
    output_file
}

fn generate_client(
    tool_path: &std::path::Path,
    specification_file: NamedTempFile,
    name: &str,
    client_version: &str,
    support_async: bool
) -> std::path::PathBuf {
    let async_name = match support_async {
        true => "async",
        false => "sync"
    };
    let name = format!("{}-{}", name, async_name);
    let user_agent = format!("{}-{}", name, client_version);
    let input_path = specification_file.path().to_str().unwrap();
    let generated_path = std::path::Path::new("generated")
        .join(name.clone());
    let generated_path_as_string = generated_path.to_str()
        .unwrap()
        .to_string();
    command(
        "npx",
        Some(&[
            "openapi-generator-cli",
            "generate",
            format!("--input-spec {}", input_path).as_str(),
            "--generator-name rust",
            format!("--output {}", generated_path_as_string).as_str(),
            format!("--http-user-agent {}", user_agent).as_str(),
            "--additional-properties=library=reqwest",
            format!("--additional-properties=supportAsync={}", support_async.to_string()).as_str(),
            format!("--additional-properties=packageName={}", name).as_str(),
            format!("--additional-properties=packageVersion={}", client_version).as_str(),
            "--additional-properties=preferUnsignedInt=true",
            "--additional-properties=topLevelApiClient=true",
            "--additional-properties=useSingleRequestParameter=true",
            "--type-mappings=string+binary=Bytes"
        ]),
        tool_path
    );
    fs::remove_file(specification_file.path()).unwrap();
    tool_path.join(generated_path)
}

pub fn read_source_file(path: &std::path::Path) -> syn::File {
    let mut file = File::open(path)
        .expect("Unable to open file");
    let mut src = String::new();
    file.read_to_string(&mut src)
        .expect("Unable to read file");
    let syntax = syn::parse_file(&src)
        .expect("Unable to parse file");
    syntax
}

fn walk_path(path: &std::path::Path, source_files: &mut HashMap<String, syn::File>) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with(".rs") {
            let module_name = path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".rs", "");
            let source_file = read_source_file(&path);
            source_files.insert(module_name, source_file);
        }
        else if path.is_dir() {
            walk_path(&path, source_files);
        }
    }
}

fn merge_model_source_files(cargo_path: &std::path::Path) -> SourceFile {
    let module_path = cargo_path
        .join("src")
        .join("models");
    let source_files = get_source_files(&module_path);
    let mut source_file = merge_source_files(&source_files);
    let path_prefix = Path::new("crate").join("models").clone();
    source_file.remove_use_items_starting_with(&path_prefix);
    let path_prefix = Path::new("models");
    source_file.trim_path_prefix_in_items(&path_prefix);
    source_file
}

fn merge_api_source_files(cargo_path: &std::path::Path) -> SourceFile {
    let module_path = cargo_path
        .join("src")
        .join("apis");
    let api_generator = ApiGenerator::new();
    let configuration_transformer = ConfigurationTransformer::new();
    let mut source_files = get_source_files(&module_path);
    for source_file in source_files.iter_mut() {
        let path_prefix = Path::new("crate");
        source_file.remove_use_items_starting_with(&path_prefix);
        let path_prefix = Path::new("super");
        source_file.remove_use_items_starting_with(&path_prefix);
        let path_prefix = Path::new("configuration");
        source_file.trim_path_prefix_in_items(&path_prefix);
        let path_prefix = Path::new("crate").join("apis").clone();
        source_file.trim_path_prefix_in_function_body(&path_prefix);
        let module = source_file.modules_mut().first_mut().unwrap();
        module.push_use_item(create_use(Path::new("crate").join("models")));
        module.push_use_item(create_use(Path::new("bytes").join("Bytes")));
        module.push_use_item(create_use(Path::new("std").join("sync").join("Arc")));
        if module.file_name() == "configuration" {
            configuration_transformer.transform(module);
        }
        if module.file_name().ends_with("_api") == false {
            continue;
        }
        let api_struct = api_generator.generate(module);
        module.push_struct_items(api_struct);
    }
    let source_file = merge_source_files(&source_files);
    source_file
}

fn merge_source_files(source_files: &Vec<SourceFile>) -> SourceFile {
    let mut output_file = SourceFile::new(
        source_files.iter()
            .flat_map(|source_file| source_file.attributes().clone())
            .collect(),
        source_files.iter()
            .flat_map(|source_file| source_file.modules().clone())
            .sorted_by(|a, b| {
                let a = a.name();
                let b = b.name();
                a.cmp(&b)
            })
            .collect::<Vec<_>>()
    );
    output_file.merge();
    output_file
}

fn get_source_files(module_path: &std::path::Path) -> Vec<SourceFile> {
    debug!("Merging module source file(s) at {}", module_path.to_str().unwrap());
    let mut source_files = HashMap::new();
    walk_path(&module_path, &mut source_files);
    debug!("Merging {} source file(s)", source_files.len());
    let mut output_files: HashMap<String, SourceFile> = HashMap::new();
    for (file_name, mut source_file) in source_files {
        if output_files.contains_key(&file_name) == false {
            let module = ModuleItems::new(file_name.clone().as_str());
            let output_file = SourceFile::new(source_file.attrs, vec![module]);
            output_files.insert(file_name.clone(), output_file);
        }
        let mut module = output_files.get_mut(&file_name)
            .unwrap()
            .modules_mut()
            .first_mut()
            .unwrap();
        for item in source_file.items {
            match item {
                syn::Item::ExternCrate(value) => {
                    module.push_extern_crate_item(value);
                }
                syn::Item::Mod(value) => {
                    if value.content.is_some() {
                        panic!("Module block not supported");
                    }
                    else {
                        // Ignoring module statement (e.g. mod my_module;)
                    }
                }
                syn::Item::Use(value) => {
                    module.push_use_item(value);
                }
                syn::Item::Struct(value) => {
                    module.push_struct(value);
                },
                syn::Item::Enum(value) => {
                    module.push_enum(value);
                }
                syn::Item::Impl(value) => {
                    module.push_impl_to_struct(&value);
                    module.push_impl_to_enum(&value);
                }
                syn::Item::Fn(value) => {
                    module.push_global_function(value);
                }
                _ => module.push_item(item),
            }
        }
    }
    let output_files = output_files.iter()
        .map(|(_, source_file)| source_file.clone())
        .collect::<Vec<_>>();
    output_files
}

fn extract_params_structs(source_file: &mut SourceFile) -> SourceFile {
    let params_structs = source_file.modules_mut().iter_mut()
        .filter_map(|module| {
            let items = module.take_items_by(
                |item| item.ident().ends_with("Params")
            );
            if items.is_empty() == false {
                Some(items)
            }
            else {
                None
            }
        })
        .flatten()
        .sorted_by(|a, b| {
            let a = a.ident();
            let b = b.ident();
            a.cmp(&b)
        })
        .collect::<Vec<_>>();
    let mut module = ModuleItems::new("params");
    module.push_use_item(create_use(Path::new("crate").join("models")));
    module.push_use_item(create_use(Path::new("bytes").join("Bytes")));
    for item in params_structs {
        let struct_item = match item {
            Item::Struct(value) => value,
            _ => panic!("Expected struct item")
        };
        module.push_struct_items(struct_item);
    }
    SourceFile::new(
        vec![],
        vec![module]
    )
}

fn create_params_builders(model_source_file: &SourceFile, params_source_file: &SourceFile) -> SourceFile {
    let mut modules = Vec::new();
    modules.append(&mut model_source_file.modules().clone());
    modules.append(&mut params_source_file.modules().clone());
    let modules = Rc::new(RefCell::new(modules));
    let mut builder = BuilderGenerator::new(modules.clone());
    builder.with_rule()
        .for_all()
        .and_all_fields()
        .then_discard_attribute("serde");
    builder.with_rule()
        .for_item("ContainerCreateRequest")
        .with_field("exposed_ports")
        .then_remap_to_vec(Path::from("ExposedPort"));
    builder.with_rule()
        .for_item("ContainerConfig")
        .with_field("exposed_ports")
        .then_remap_to_vec(Path::from("ExposedPort"));
    builder.with_rule()
        .for_item("ContainerCreateRequest")
        .with_field("volumes")
        .then_remap_to_vec(Path::new("PathBuf"));
    builder.with_rule()
        .for_item("ContainerConfig")
        .with_field("volumes")
        .then_remap_to_vec(Path::new("PathBuf"));
    builder.with_rule()
        .for_item("ServiceSpecMode")
        .with_field("global")
        .then_map(Path::new("String"));
    builder.with_rule()
        .for_item("ServiceSpecMode")
        .with_field("global_job")
        .then_map(Path::new("String"));
    builder.with_rule()
        .for_item("ClusterVolumeSpecAccessMode")
        .with_field("mount_volume")
        .then_map(Path::new("String"));
    let borrowed_modules = modules.borrow();
    let params_structs = borrowed_modules.iter()
        .filter_map(|module| {
            let items = module.find_items_by(
                |item| item.ident().ends_with("Params")
            );
            if items.is_empty() == false {
                Some(items)
            }
            else {
                None
            }
        })
        .flatten()
        .sorted_by(|a, b| {
            let a = a.ident();
            let b = b.ident();
            a.cmp(&b)
        })
        .collect::<Vec<_>>();
    let mut module_builders = ModuleItems::new("builders");
    module_builders.push_use_item(create_use_as_glob(Path::new("crate").join("models")));
    module_builders.push_use_item(create_use_as_glob(Path::new("crate").join("params")));
    module_builders.push_use_item(create_use(Path::new("bytes").join("Bytes")));
    module_builders.push_use_item(create_use(Path::new("std").join("collections").join("HashMap")));
    module_builders.push_use_item(create_use(Path::new("std").join("path").join("PathBuf")));
    params_structs.iter()
        .flat_map(|item| {
            match item {
                Item::Struct(value) => {
                    builder.generate(&syn::Item::Struct(value.item().clone()))
                }
                _ => panic!("Unexpected type !")
            }
        })
        .unique_by(|item| item.ident())
        .for_each(|item| module_builders.push_struct_items(item));
    SourceFile::new(vec![], vec![module_builders])
}