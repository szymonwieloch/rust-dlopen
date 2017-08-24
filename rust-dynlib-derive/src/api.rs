use syn::{Body, VariantData, Field, Ty, MetaItem, Lit, DeriveInput};
use proc_macro::TokenStream;
use quote;
use super::common::{get_fields, symbol_name};

pub fn impl_library_api(ast: &DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields = get_fields(ast, "LibraryApi");

    let tok_iter = fields.iter().map(field_to_tokens);
    let q = quote! {
        impl<'a> LibraryApi<'a> for #name<'a> {
            unsafe fn load(lib: &'a ::dynlib::symbor::SymBorLib) -> Result<#name<'a>,::dynlib::Error> {
                Ok(#name {
                #(#tok_iter),*
                })
            }
        }
    };

    //panic!("{}", q.as_str());
    q
}


fn field_to_tokens(field: &Field) -> quote::Tokens {
    let field_name = &field.ident;
    let symbol_name: &str = symbol_name(field);

    //panic!("type_name = {}, {:?}", field_type_name, field);

    quote! {
        #field_name: {
            let raw_result = lib.ptr_or_null_cstr::<()>(
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
            );
            dynlib::symbor::FromRawResult::from_raw_result(raw_result)?
        }
    }

}