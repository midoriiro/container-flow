use std::collections::HashMap;
use itertools::Itertools;
use syn::punctuated::Punctuated;
use syn::{Block, ExprPath, FieldMutability, Fields, FieldsNamed, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, ItemStruct, Pat, PatType, Receiver, ReturnType, Signature, Type, TypePath, TypeReference, Visibility};
use ast_shaper::items::fn_item::FnItem;
use ast_shaper::items::item::{Item, ItemTrait};
use ast_shaper::items::module_item::ModuleItem;
use ast_shaper::items::struct_item::StructItem;
use ast_shaper::utils::create_ident;
use ast_shaper::utils::name_conventions::NamingConventions;
use ast_shaper::utils::path::Path;
use ast_shaper::utils::punctuated::PunctuatedExt;
use ast_shaper::utils::statement::{Expr, Statement};

pub struct ApiGenerator;

impl ApiGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, module: &mut ModuleItem) -> StructItem {
        let file_name = module.file_name().clone();
        let file_name = file_name.replace("_api", "");
        let mut items = module.take_items_by(|item| {
            match item {
                Item::Fn(value) => value.ident().contains(&file_name),
                _ => false
            }
        });
        let items = items.iter_mut()
            .map(|item| {
                match item {
                    Item::Fn(value) => {
                        let ident = value.ident();
                        let ident = ident.replace(&format!("{}_", file_name), "");
                        value.item.signature_mut().ident = create_ident(&ident);
                        value.clone()
                    },
                    _ => panic!("Expected function item")
                }
            })
            .collect::<Vec<_>>();
        let ident = create_ident(format!("{}Api", NamingConventions::to_pascal_case(&file_name)).as_str());
        let item_struct = self.generate_struct_item(&ident);
        let struct_impl_item = self.generate_struct_impl_item(&ident, &items);
        StructItem::new(
            item_struct,
            vec![struct_impl_item],
        )
    }

    fn generate_struct_item(
        &self,
        ident: &Ident,
    ) -> ItemStruct {
        ItemStruct {
            attrs: Default::default(),
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident: ident.clone(),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(syn::Field {
                    attrs: vec![],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(create_ident("configuration")),
                    colon_token: Default::default(),
                    ty: Type::Path(TypePath {
                        qself: None,
                        path: Path::new("Arc").with(Path::new("Configuration")).to_syn_path(),
                    }),
                }),
            }),
            semi_token: Default::default(),
        }
    }

    fn generate_struct_impl_item(
        &self,
        ident: &Ident,
        functions: &Vec<FnItem>
    ) -> ItemImpl {
        let mut methods = vec![Self::generate_new_method()];
        methods.append(&mut functions.iter()
           .map(|function| Self::transform_function_to_method(function))
           .collect::<Vec<_>>()
        );
        ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: None,
            self_ty: Box::new(Type::Path(TypePath {
                qself: None,
                path: Path::from(ident.clone()).to_syn_path(),
            })),
            brace_token: Default::default(),
            items: methods,
        }
    }

    fn generate_new_method() -> ImplItem {
        let item = ImplItem::Fn(ImplItemFn {
            attrs: vec![],
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: create_ident("new"),
                generics: Default::default(),
                paren_token: Default::default(),
                inputs: Punctuated::single(FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Path(ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: Path::new("configuration").to_syn_path(),
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(Type::Path(TypePath {
                        qself: None,
                        path: Path::new("Arc").with(Path::new("Configuration")).to_syn_path(),
                    })),
                })),
                variadic: None,
                output: ReturnType::Type(Default::default(), Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path::new("Self").to_syn_path(),
                })))
            },
            block: Block {
                brace_token: Default::default(),
                stmts: vec![
                    Statement::implicit_return(
                        Expr::Stmt(Statement::struct_literal(
                            Path::new("Self"),
                            vec![(
                                "configuration".to_string(),
                                Expr::Path(Path::new("configuration"))
                            )].into_iter().collect::<HashMap<_, _>>()
                        ))
                    )
                ],
            },
        });
        item
    }

    fn transform_function_to_method(item: &FnItem) -> ImplItem {
        let mut signature = item.signature().clone();
        let first_argument = signature.inputs.first_mut().unwrap();
        let field = match first_argument {
            FnArg::Typed(ref value) => {
                match *value.pat {
                    Pat::Ident(ref value) => Path::from(value.ident.clone()),
                    _ => panic!("Expected ident argument: {:?}", value.pat)
                }
            }
            _ => panic!("Expected typed argument")
        };
        *first_argument = FnArg::Receiver(Receiver {
            attrs: vec![],
            reference: Some((Default::default(), None)),
            mutability: Some(Default::default()),
            self_token: Default::default(),
            colon_token: None,
            ty: Box::new(Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: None,
                mutability: Some(Default::default()),
                elem: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path::new("Self").to_syn_path(),
                })),
            }))
        });
        let mut block = item.block().clone();
        Self::transform_function_body(&mut block, field.clone());
        ImplItem::Fn(ImplItemFn {
            attrs: item.attributes().clone(),
            vis: item.visibility().clone(),
            defaultness: Default::default(),
            sig: signature,
            block,
        })
    }

    fn transform_function_body(block: &mut Block, field: Path) {
        let mut context = ast_shaper::functions::transform::create_context(move |expr| {
            match expr {
                syn::Expr::Field(value) => {
                    let base = match *value.base {
                        syn::Expr::Path(ref value) => value.path.clone().into(),
                        _ => panic!("Expected path expression")
                    };
                    if base != field {
                        return false;
                    }
                    *value.base = Expr::Stmt(Statement::access_field(
                        Path::new("self"),
                        base
                    )).to_expr();
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