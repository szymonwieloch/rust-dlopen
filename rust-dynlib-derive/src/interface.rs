use syn::{Body, VariantData, Field, Ty, MetaItem, Lit, DeriveInput, BareFnTy, BareFnArg, FunctionRetTy};
use proc_macro::TokenStream;
use quote;
use super::common::{get_fields, symbol_name, has_marker_attr};

const TRAIT_NAME: &str = "LibraryInterface";

pub fn impl_library_interface(ast: &DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let fields = get_fields(ast, TRAIT_NAME);
    let functions = implement_structure_functions(ast);
    let field_iter = fields.iter().map(field_txt);
    let q = quote! {
        impl LibraryInterface for #struct_name {
            unsafe fn load(lib: & ::dlopen::DlOpen) -> Result<Self, ::dlopen::Error> {
               Ok(Self {
                    #(#field_iter),*
               })
            }
        }

        #functions
    };
    //panic!("{}", q.as_str());
    q
}

fn field_txt(field: &Field) -> quote::Tokens {
    let field_name = &field.ident;
    let symbol_name: &str = symbol_name(field);

    //panic!("type_name = {}, {:?}", field_type_name, field);
    match field.ty{
        Ty::BareFn(_) =>  quote! {
            #field_name : lib.symbol_cstr(
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
            )?
        },
        Ty::Ptr(_) => quote! {
             #field_name : lib.pointer_cstr(
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(#symbol_name, "\0").as_bytes())
            )?
        },
        _ => panic! ("{} implementations can contain only functions and pointers, field \"{}\" does not conform", TRAIT_NAME, symbol_name)
    }

}

fn implement_structure_functions(ast: &DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let fields = get_fields(ast, TRAIT_NAME);
    let func_iter = fields.iter()
        .filter_map(|f| if let Ty::BareFn(ref val) = f.ty{Some((val, f))} else {None})
        .map(|t |bare_fn_caller(t.0, t.1));
    quote! {
        impl #struct_name {
            #(#func_iter),*
        }
    }
}

fn bare_fn_caller(fun: & Box<BareFnTy>, field: &Field) -> quote::Tokens {
    let fname = &field.ident;
    let inputs = fun.inputs.iter().map(|i|{
        let name = i.name.as_ref().expect("All function parameters need to be named");
        let arg_type = &i.ty;

        quote!{
           , #name: #arg_type
        }
    });
    let arguments = fun.inputs.iter().map(|i| i.name.as_ref().expect("All function parameters need to be named"));
    let output = match fun.output {
        FunctionRetTy::Default => quote!{},
        FunctionRetTy::Ty(ref ty) => quote!{-> #ty}
    };
    quote! {
        pub unsafe fn #fname (&self #(#inputs)*) #output {
            (self.#fname)(#(#arguments),*)
        }
    }
}