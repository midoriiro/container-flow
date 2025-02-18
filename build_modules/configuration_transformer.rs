use crate::build_modules::items::{ItemTrait, ModuleItems};
use crate::build_modules::utils::expr::StatementWalker;
use crate::build_modules::utils::path::Path;
use crate::build_modules::utils::statement::{Expr, Statement};
use syn::punctuated::Punctuated;
use syn::{Block, Field, Fields, FieldsNamed, ItemStruct, Member, Token};
use pipewire_common::debug;

pub struct ConfigurationTransformer;

impl ConfigurationTransformer {
    pub fn new() -> Self {
        Self
    }

    pub fn transform(&self, module: &mut ModuleItems) {
        const FIELDS_TO_REMOVE: &[&str] = &[
            "basic_auth",
            "oauth_access_token",
            "bearer_access_token",
            "api_key",
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
        Self::transform_fields(configuration_item.item_mut(), FIELDS_TO_REMOVE);
        let new_function = configuration_item.impl_items_mut().iter_mut()
            .flat_map(|impl_item| impl_item.functions_mut())
            .find(|function| function.ident() == "new")
            .unwrap();
        Self::transform_init_function_body(new_function.block_mut(), FIELDS_TO_REMOVE);
        let default_function = configuration_item.impl_items_mut().iter_mut()
            .filter_map(|impl_item| {
                let trait_impl = impl_item.trait_impl();
                match trait_impl {
                    None => None,
                    Some(value) => {
                        if value == Path::new("Default") {
                            Some(impl_item.functions_mut())
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
        Self::transform_init_function_body(default_function.block_mut(), FIELDS_TO_REMOVE);
    }
    
    fn transform_fields(item: &mut ItemStruct, fields_to_remove: &[&str]) {
        let fields = item.fields.iter_mut()
            .filter(|field| {
                fields_to_remove.contains(&field.ident.as_ref().unwrap().to_string().as_str()) == false
            })
            .map(|field| field.clone())
            .collect::<Punctuated<Field, Token![,]>>();
        item.fields = Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: fields
        });
    }

    fn transform_init_function_body(block: &mut Block, fields_to_remove: &[&str]) {
        for statement in block.stmts.iter_mut() {
            StatementWalker::walk(statement, &mut |expr| {
                match expr {
                    syn::Expr::Struct(ref mut value) => {
                        let fields = value.fields.iter_mut()
                            .filter(|field| {
                                match &field.member {
                                    Member::Named(value) => {
                                        fields_to_remove.contains(&value.to_string().as_str()) == false
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
        }
    }

    fn transform_function_body(block: &mut Block, field: &Path) {
        for statement in block.stmts.iter_mut() {
            StatementWalker::walk(statement, &mut |expr| {
                match expr {
                    syn::Expr::Field(ref mut value) => {
                        let base = match *value.base {
                            syn::Expr::Path(ref value) => value.path.clone().into(),
                            _ => panic!("Expected path expression")
                        };
                        if base != *field {
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
        }
    }
}