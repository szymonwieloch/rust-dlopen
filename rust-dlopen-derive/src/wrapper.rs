use syn::{Body, VariantData, Field, Ty, MetaItem, Lit, DeriveInput};
use proc_macro::TokenStream;
use quote;
use super::common::{get_fields, symbol_name, find_marker_attr};



pub fn impl_library_wrapper(ast: &DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields = get_fields(ast, "LibraryWrapper");

    let drop_idx = find_drop_field(fields);


    let tok_iter = fields.iter().enumerate().map(|t| field_to_tokens(t.1, t.0 == drop_idx));
    let q = quote! {
        impl LibraryWrapper for #name {
            unsafe fn load(lib_name: &str) -> Result<#name,dlopen::Error> {
                let cname = CString::new(lib_name)?;
                let handle = libc::dlopen(cname);
                Ok(#name {
                #(#tok_iter),*
                })
            }
        }
    };

    //panic!("{}", q.as_str());
    q
}

const DLOPEN_DROP: &str = "dlopen_drop";

fn find_drop_field(fields: &Vec<Field>) -> usize {
    let marked_fields: Vec<usize> = fields.iter().enumerate().filter(|t| find_marker_attr(t.1, DLOPEN_DROP)).map(|t| t.0).collect();
    match marked_fields.len() {
        1 => return *marked_fields.first().unwrap(),
        num if num>1 => panic!("{} attribute can be assigned to max 1 fields", DLOPEN_DROP),
        _ => ()
    };

    let ld_fields: Vec<usize> = fields.iter().enumerate().filter(|t|is_library_drop(t.1)).map(|t| t.0).collect();
    match ld_fields.len() {
        1 =>  return *ld_fields.first().unwrap(),
        0 => panic!("Add LibraryDrop to your structure od mark it with {}", DLOPEN_DROP),
        _ => panic!("LibraryWrapper implementations can have only one LibraryDrop field")
    }
}

fn is_library_drop(field: &Field) -> bool {
    match field.ty {
        Ty::Path(ref ident, ref path) => {
            match path.segments.last() {
                Some(ref last) => last.ident.as_ref() == "LibraryWrapper",
                None => false
            }
        },
        _=> false

    }
}

fn field_to_tokens(field: &Field, is_drop: bool) -> quote::Tokens {
    let field_name = &field.ident;
    let symbol_name: &str = symbol_name(field);

    //panic!("type_name = {}, {:?}", field_type_name, field);
    if is_drop {
        quote! {
            #field_name : libdrop
        }
    }else {
        quote! {
            #field_name: {
                let _ = dlerror();
                let cname = std::ffi::CString::new(name)?;
                let symbol = dlsym(handle, cname.as_ptr());
                //This can be either error or just the library has a NULl pointer - legal
                if symbol.is_null() {
                    let msg = dlerror();
                    return Err(if msg.is_null() {
                        //this is correct behavior but we can't convert NULL to reference
                        dlopen::Error::NullPointer
                    } else {
                        //this is just error
                        dlopen::Error::DlError(dlopen::DlError::from_ptr(msg))
                    })
                }
                std::mem::transmute(symbol)
            }
        }
    }

}