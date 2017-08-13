extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{Body, VariantData, Field, Ty, MetaItem, Lit};
use proc_macro::TokenStream;

#[proc_macro_derive(LibraryApi, attributes(dlopen_name))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_library_api(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_library_api(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let vd = match ast.body {
        Body::Enum(_) => panic!("LibraryApi can be only implemented for structures"),
        Body::Struct(ref val) => val
    };
    let fields = match vd {
        &VariantData::Struct(ref f) => f,
        &VariantData::Tuple(_) | &VariantData::Unit => panic!("LibraryApi can be only implemented for structures")
    };

    let tok_iter = fields.iter().map(field_to_tokens);
    let q = quote! {
        impl<'a> LibraryApi<'a> for #name<'a> {
            unsafe fn load(lib: &'a dlopen::Library) -> Result<#name<'a>,dlopen::Error> {
                Ok(#name {
                #(#tok_iter),*
                })
            }
        }
    };

    //panic!("{}", q.as_str());
    q
}

fn symbol_name(field: &Field) -> &str {
    for attr in field.attrs.iter() {
        match attr.value {
            MetaItem::NameValue(ref ident, ref it) => {
                if ident.as_ref() == "dlopen_name" {
                    return match it {
                        &Lit::Str(ref val, ref style) => {
                            val.as_ref()
                        },
                        _ => panic!("dlopen_name attribute must be a string")
                    }
                }
            },
            _ => continue
        }
    }
    //not found, so use field name
    match field.ident {
        Some(ref val) => val.as_ref(),
        None => panic!("All structure fields need to be identificable")
    }
}

fn field_to_tokens(field: &Field) -> quote::Tokens {
    let field_name = &field.ident;
    let symbol_name: &str = symbol_name(field);

    //panic!("type_name = {}, {:?}", field_type_name, field);

    // Some fields supports null pointers, some do not
    //but the TryFrom trait is still unstable and conditional conversion is not yet supported
    //TODO: change this once TryFrom is stable
    quote! {
        #field_name: {
            let raw = lib.raw(#symbol_name)?;
            dlopen::FromRawPointer::from_raw_ptr(raw)?
        }
    }

}