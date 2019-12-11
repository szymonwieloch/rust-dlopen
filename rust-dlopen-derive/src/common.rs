use syn::{Data, DeriveInput, Field, Fields, FieldsNamed, Lit, Meta};

pub fn symbol_name(field: &Field) -> String {
    match find_str_attr_val(field, "dlopen_name") {
        Some(val) => val,
        None => {
            //not found, so use field name
            match field.ident {
                Some(ref val) => val.to_string(),
                None => panic!("All structure fields need to be identifiable"),
            }
        }
    }
}

pub fn find_str_attr_val(field: &Field, attr_name: &str) -> Option<String> {
    for attr in field.attrs.iter() {
        match attr.parse_meta() {
            Ok(Meta::NameValue(ref meta)) => {
                if meta.path.is_ident(attr_name) {
                    return match meta.lit {
                        Lit::Str(ref val, ..) => Some(val.value()),
                        _ => panic!("{} attribute must be a string", attr_name),
                    };
                }
            }
            _ => continue,
        }
    }
    None
}

pub fn has_marker_attr(field: &Field, attr_name: &str) -> bool {
    for attr in field.attrs.iter() {
        match attr.parse_meta() {
            Ok(Meta::Path(ref val)) => {
                if val.is_ident(attr_name) {
                    return true;
                }
            }
            _ => continue,
        }
    }
    false
}

pub fn get_fields<'a>(ast: &'a DeriveInput, trait_name: &str) -> &'a FieldsNamed {
    let vd = match ast.data {
        Data::Struct(ref val) => val,
        Data::Enum(_) | Data::Union(_) => {
            panic!("{} can be only implemented for structures", trait_name)
        }
    };
    match vd.fields {
        Fields::Named(ref f) => f,
        Fields::Unnamed(_) | Fields::Unit => {
            panic!("{} can be only implemented for structures", trait_name)
        }
    }
}
