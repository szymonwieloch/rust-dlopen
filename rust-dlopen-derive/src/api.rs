use syn::{Field, DeriveInput};
use syn;
use super::common::{get_fields, symbol_name};

pub fn impl_library_api(ast: &DeriveInput) -> syn::export::TokenStream2 {
    let name = &ast.ident;
    let fields = get_fields(ast, "SymBorApi");

    let tok_iter = fields.named.iter().map(field_to_tokens);
    let q = quote! {
        impl<'a> SymBorApi<'a> for #name<'a> {
            unsafe fn load(lib: &'a ::dlopen::symbor::Library) -> ::std::result::Result<#name<'a>,::dlopen::Error> {
                ::std::result::Result::Ok(#name {
                #(#tok_iter),*
                })
            }
        }
    };

    //panic!("{}", q.as_str());
    q
}


fn field_to_tokens(field: &Field) -> syn::export::TokenStream2 {
    let field_name = &field.ident;
    let symbol_name = symbol_name(field);

    //panic!("type_name = {}, {:?}", field_type_name, field);

    quote! {
        #field_name: {
            let raw_result = lib.ptr_or_null_cstr::<()>(
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
            );
            ::dlopen::symbor::FromRawResult::from_raw_result(raw_result)?
        }
    }

}
