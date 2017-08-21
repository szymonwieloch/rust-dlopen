use syn::{Body, VariantData, Field, Ty, MetaItem, Lit, DeriveInput, Visibility};
use proc_macro::TokenStream;
use quote;
use super::common::{get_fields, symbol_name, has_marker_attr};

const ALLOW_NULL: &str = "dynlib_allow_null";
const TRAIT_NAME: &str = "WrapperApi";

pub fn impl_wrapper_api(ast: &DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let fields = get_fields(ast, TRAIT_NAME);
    let generics = &ast.generics;
    //make sure that all fields are private - panic otherwise
    //make sure that all fields are identificable - panic otherwise
    for field in fields.iter(){
        let _ = field.ident.as_ref().expect("All fields of structures deriving WrapperAPI need to be identificable");
        match field.vis {
            Visibility::Inherited => (),
            _ => panic!("All fields of structures deriving {} need to be private and '{}' is not",
                        TRAIT_NAME, field.ident.as_ref().unwrap())
        }
    }

    let field_iter = fields.iter().map(field_to_tokens);
    let wrapper_iter = fields.iter().filter_map(field_to_wrapper);
    let q = quote! {
        impl #generics WrapperApi for #struct_name #generics {
            unsafe fn load(lib: & ::dynlib::lowlevel::DynLib ) -> Result<Self, ::dynlib::Error> {
                Ok(Self{
                    #(#field_iter),*
                })
            }
        }

        impl #generics #struct_name #generics {
            #(#wrapper_iter), *
        }
    };

    //panic!("{}", q.as_str());
    q
}

fn field_to_tokens(field: &Field) -> quote::Tokens {
    let allow_null = has_marker_attr(field, ALLOW_NULL);
    match field.ty {
        Ty::BareFn(_) | Ty::Rptr(_, _) => {
            if allow_null {
                panic!("Only pointers can have the '{}' attribute assigned", ALLOW_NULL);
            }
            normal_field(field)
        },
        Ty::Ptr(_) => if has_marker_attr(field, ALLOW_NULL) {
            allow_null_field(field)
        } else {
            normal_field(field)
        },
        _ => panic!("Only bare functions, references an pointers are allowed in structures implementing WrapperApi trait")
    }
}


fn normal_field(field: &Field) -> quote::Tokens {
    let field_name = &field.ident;
    let symbol_name: &str = symbol_name(field);
    quote! {
        #field_name : lib.symbol_cstr(
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
        )?
    }
}


fn allow_null_field(field: &Field) -> quote::Tokens {
    let field_name = &field.ident;
    let symbol_name: &str = symbol_name(field);
    quote! {
        #field_name : lib.symbol_cstr(
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
        )?
    }
}

fn field_to_wrapper(field: &Field) -> Option<quote::Tokens> {
    let ident = &field.ident;
    match &field.ty {
        &Ty::BareFn(_) => {
            None
        },
        &Ty::Rptr(_, ref mut_ty) => {
            let ty = &mut_ty.ty;
            let mutability = &mut_ty.mutability;
            Some(quote! {
                pub unsafe fn #ident (& #mutability self) -> & #mutability #ty {
                    self.#ident
                }
            })
        },
        &Ty::Ptr(_) => None,
        _ => panic!("Unknown field type, this should not happen!")
    }
}