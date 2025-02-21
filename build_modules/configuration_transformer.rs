use ast_shaper::items::item::ItemTrait;
use ast_shaper::items::module_item::ModuleItem;
use ast_shaper::utils::path::Path;
use ast_shaper::utils::statement::{Expr, Statement};
use syn::punctuated::Punctuated;
use syn::{Block, Field, Fields, FieldsNamed, ItemStruct, Member, Token};

pub struct ConfigurationTransformer;

impl ConfigurationTransformer {
    pub fn new() -> Self {
        Self
    }

    pub fn transform(&self, module: &mut ModuleItem) {
        let fields_to_remove = vec![
            "basic_auth".to_string(),
            "oauth_access_token".to_string(),
            "bearer_access_token".to_string(),
            "api_key".to_string(),
        ];
        let file_name = module.file_name().clone();
        if file_name != "configuration" {
            return;
        }
        module.take_items_by(|item| item.ident() == "BasicAuth");
        module.take_items_by(|item| item.ident() == "ApiKey");
        let configuration_item = module
            .find_item_mut_by(|item| item.ident() == "Configuration")
            .unwrap()
            .as_struct_mut()
            .unwrap();
        Self::transform_fields(&mut configuration_item.item, fields_to_remove.clone());
        let new_function = configuration_item.impl_items.iter_mut()
            .flat_map(|impl_item| &mut impl_item.functions)
            .find(|function| function.ident() == "new")
            .unwrap();
        Self::transform_init_function_body(new_function.block_mut(), fields_to_remove.clone());
        let default_function = configuration_item.impl_items.iter_mut()
            .filter_map(|impl_item| {
                let trait_impl = &impl_item.item.trait_;
                match trait_impl {
                    None => None,
                    Some((_, value, _)) => {
                        if *value == Path::new("Default").to_syn_path() {
                            Some(&mut impl_item.functions)
                        }
                        else {
                            None
                        }
                    }
                }
            })
            .flatten()
            .find(|function| function.ident() == "default")
            .unwrap();
        Self::transform_init_function_body(default_function.block_mut(), fields_to_remove.clone());
    }
    
    fn transform_fields(item: &mut ItemStruct, fields_to_remove: Vec<String>) {
        let fields = item.fields.iter_mut()
            .filter(|field| {
                fields_to_remove.contains(&field.ident.as_ref().unwrap().to_string()) == false
            })
            .map(|field| field.clone())
            .collect::<Punctuated<Field, Token![,]>>();
        item.fields = Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: fields
        });
    }

    fn transform_init_function_body(block: &mut Block, fields_to_remove: Vec<String>) {
        let mut context = ast_shaper::functions::transform::create_context(move |expr| {
            match expr {
                syn::Expr::Struct(value) => {
                    let fields = value.fields.iter_mut()
                        .filter(|field| {
                            match &field.member {
                                Member::Named(value) => {
                                    fields_to_remove.contains(&value.to_string()) == false
                                }
                                _ => panic!("Expected named field"),
                            }
                        })
                        .map(|field| field.clone())
                        .collect::<Punctuated<syn::FieldValue, Token![,]>>();
                    value.fields = fields;
                    true
                }
                _ => false
            }
        });
        for statement in block.stmts.iter_mut() {
            ast_shaper::functions::transform::from_stmt(statement, &mut context);
        }
    }

    fn transform_function_body(block: &mut Block, field: Path) {
        let mut context = ast_shaper::functions::transform::create_context(move |expr| {
            match expr {
                syn::Expr::Field(ref mut value) => {
                    let base = match *value.base {
                        syn::Expr::Path(ref value) => value.path.clone().into(),
                        _ => panic!("Expected path expression")
                    };
                    if base != field {
                        return false;
                    }
                    value.base = Box::new(Expr::Stmt(Statement::access_field(
                        Path::new("self"),
                        base
                    )).to_expr());
                    true
                }
                _ => false
            }
        });
        for statement in block.stmts.iter_mut() {
            ast_shaper::functions::transform::from_stmt(statement, &mut context);
        }
    }
}