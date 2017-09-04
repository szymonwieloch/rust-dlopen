use syn::{Field, DeriveInput};
use quote;
use super::common::{get_fields};

const TRATIT_NAME: &str = "WrapperMultiApi";

pub fn impl_wrapper_multi_api(ast: &DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;
    let fields = get_fields(ast, TRATIT_NAME);

    let tok_iter = fields.iter().map(field_to_tokens);
    let q = quote! {
        impl #generics WrapperMultiApi for #name #generics{}

         impl #generics ::dlopen::wrapper::WrapperApi for # name #generics{
            unsafe fn load(lib: & ::dlopen::raw::Library) -> Result<Self,::dlopen::Error> {
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

    //panic!("type_name = {}, {:?}", field_type_name, field);

    quote! {
        #field_name: ::dlopen::wrapper::WrapperApi::load(&lib)?
    }

}