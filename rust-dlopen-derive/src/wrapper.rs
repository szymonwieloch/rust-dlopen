use syn::{Field, Type, DeriveInput, Visibility, BareFnArg, TypePtr};
use syn;
use super::common::{get_fields, symbol_name, has_marker_attr};
const ALLOW_NULL: &str = "dlopen_allow_null";
const TRAIT_NAME: &str = "WrapperApi";

pub fn impl_wrapper_api(ast: &DeriveInput) -> syn::export::TokenStream2 {
    let struct_name = &ast.ident;
    let fields = get_fields(ast, TRAIT_NAME);
    let generics = &ast.generics;
    //make sure that all fields are private - panic otherwise
    //make sure that all fields are identifiable - panic otherwise
    for field in fields.named.iter(){
        let _ = field.ident.as_ref().expect("All fields of structures deriving WrapperAPI need to be identificable");
        match field.vis {
            Visibility::Inherited => (),
            _ => panic!("All fields of structures deriving {} need to be private and '{}' is not",
                        TRAIT_NAME, field.ident.as_ref().unwrap())
        }
    }

    let field_iter = fields.named.iter().map(field_to_tokens);
    let wrapper_iter = fields.named.iter().filter_map(field_to_wrapper);
    let q = quote! {
        impl #generics WrapperApi for #struct_name #generics {
            unsafe fn load(lib: & ::dlopen::raw::Library ) -> ::std::result::Result<Self, ::dlopen::Error> {
                Ok(Self{
                    #(#field_iter),*
                })
            }
        }

        #[allow(dead_code)]
        impl #generics #struct_name #generics {
            #(#wrapper_iter)*
        }
    };

    //panic!("{}", q.as_str());
    q
}

fn field_to_tokens(field: &Field) -> syn::export::TokenStream2 {
    let allow_null = has_marker_attr(field, ALLOW_NULL);
    match field.ty {
        Type::BareFn(_) | Type::Reference(_) => {
            if allow_null {
                panic!("Only pointers can have the '{}' attribute assigned", ALLOW_NULL);
            }
            normal_field(field)
        },
        Type::Ptr(ref ptr) => if allow_null {
            allow_null_field(field, ptr)
        } else {
            normal_field(field)
        },
        _ => panic!("Only bare functions, references an pointers are allowed in structures implementing WrapperApi trait")
    }
}


fn normal_field(field: &Field) -> syn::export::TokenStream2 {
    let field_name = &field.ident;
    let symbol_name = symbol_name(field);
    quote! {
        #field_name : lib.symbol_cstr(
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
        )?
    }
}


fn allow_null_field(field: &Field, ptr: &TypePtr) -> syn::export::TokenStream2 {
    let field_name = &field.ident;
    let symbol_name = symbol_name(field);
    let null_fun = match ptr.mutability {
        Some(_) => quote!{null},
        None => quote!{null_mut}
    };
    quote! {
        #field_name : match lib.symbol_cstr(
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
        ) {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => match err {
                ::dlopen::Error::NullSymbol => ::std::ptr:: #null_fun (),
                _ => return ::std::result::Result::Err(err)
            }
        }
    }
}

fn field_to_wrapper(field: &Field) -> Option<syn::export::TokenStream2> {
    let ident = &field.ident;
    match &field.ty {
        &Type::BareFn(ref fun) => {
            if fun.variadic.is_some() {
                None
            } else {
                let output = &fun.output;
                let unsafety = &fun.unsafety;
                let arg_iter = fun.inputs.iter().map(|a| fun_arg_to_tokens(a, &ident.as_ref().unwrap().to_string()));
                let arg_names = fun.inputs.iter().map(|a| match a.name {
                    ::std::option::Option::Some((ref arg_name, _)) => arg_name,
                    ::std::option::Option::None => panic!("This should never happen")
                });
                Some(quote! {
                    pub #unsafety fn #ident (&self, #(#arg_iter),* ) #output {
                        (self.#ident)(#(#arg_names),*)
                    }
                })
            }
        },
        &Type::Reference(ref ref_ty) => {
            let ty = &ref_ty.elem;
            let mut_acc = match ref_ty.mutability {
                Some(_token) => {
                    let mut_ident = &format!("{}_mut", ident.as_ref().unwrap().to_string());
                    let method_name = syn::Ident::new(&mut_ident, ident.as_ref().unwrap().span());
                    Some(quote!{
                        pub fn #method_name (&mut self) -> &mut #ty {
                            self.#ident
                        }
                    })
                },
                None => None,
            };
            //constant accessor
            let const_acc = quote! {
                pub fn #ident (&self) -> & #ty {
                    self.#ident
                }
            };

            Some(quote! {
            #const_acc
            #mut_acc
            })
        },
        &Type::Ptr(_) => None,
        _ => panic!("Unknown field type, this should not happen!")
    }
}

fn fun_arg_to_tokens(arg: &BareFnArg, function_name: &str) -> syn::export::TokenStream2 {
    let arg_name = match arg.name {
        Some(ref val) => &val.0,
        None => panic!("Function {} has an unnamed argument.", function_name)
    };
    let ty = &arg.ty;
    quote!{
        #arg_name: #ty
    }
}