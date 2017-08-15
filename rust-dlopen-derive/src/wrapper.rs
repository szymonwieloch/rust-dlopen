use syn::{Body, VariantData, Field, Ty, MetaItem, Lit, DeriveInput};
use proc_macro::TokenStream;
use quote;
use super::common::{get_fields, symbol_name, has_marker_attr};

const DLOPEN_DROP: &str = "dlopen_drop";
const DLOPEN_ALLOW_NULL: &str = "dlopen_allow_null";
const TRAIT_NAME: &str = "LibraryWrapper";

pub fn impl_library_wrapper(ast: &DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let fields = get_fields(ast, TRAIT_NAME);

    let drop_idx = find_drop_field(fields);
    let drop_field = &fields[drop_idx].ident;


    let field_iter = fields.iter().enumerate().filter(|t| t.0 != drop_idx).map(|t| normal_field(t.1));
    let q = quote! {
        impl LibraryWrapper for #struct_name {
            unsafe fn load_cstr(lib_name: &::std::ffi::CStr) -> Result<Self,dlopen::Error> {
                let lib = ::dlopen::DlOpen::open_cstr(lib_name)?;
                Ok(Self{
                    #(#field_iter)*
                    #drop_field: lib.into_drop()
                })
            }
        }
    };

    //panic!("{}", q.as_str());
    q
}

fn find_drop_field(fields: &Vec<Field>) -> usize {
    let marked_fields: Vec<usize> = fields.iter().enumerate().filter(|t| has_marker_attr(t.1, DLOPEN_DROP)).map(|t| t.0).collect();
    match marked_fields.len() {
        1 => return *marked_fields.first().unwrap(),
        num if num>1 => panic!("{} attribute can be assigned to max 1 fields", DLOPEN_DROP),
        _ => ()
    };

    let ld_fields: Vec<usize> = fields.iter().enumerate().filter(|t|is_library_drop(t.1)).map(|t| t.0).collect();
    match ld_fields.len() {
        1 =>  return *ld_fields.first().unwrap(),
        0 => panic!("Add DlDrop to your structure od mark it with {}", DLOPEN_DROP),
        _ => panic!("LibraryWrapper implementations can have only one DlDrop field")
    }
}

fn is_library_drop(field: &Field) -> bool {
    match field.ty {
        Ty::Path(ref ident, ref path) => {
            match path.segments.last() {
                Some(ref last) => last.ident.as_ref() == "DlDrop",
                None => false
            }
        },
        _=> false

    }
}

fn normal_field(field: &Field) -> quote::Tokens {
    let field_name = &field.ident;
    let symbol_name: &str = symbol_name(field);

    //panic!("type_name = {}, {:?}", field_type_name, field);
    if has_marker_attr(field, DLOPEN_ALLOW_NULL){
        quote! {
            #field_name : lib.pointer_cstr(
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
            )?,
        }
    } else {
        quote! {
            #field_name : lib.symbol_cstr(
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
            )?,
        }
    }
}