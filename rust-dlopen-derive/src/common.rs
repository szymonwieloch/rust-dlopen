use syn::{Field, DeriveInput, Body, VariantData, Lit, MetaItem};

pub fn symbol_name(field: &Field) -> &str {
    match find_str_attr_val(field, "dlopen_name") {
        Some(val) => val,
        None => //not found, so use field name
            match field.ident {
                Some(ref val) => val.as_ref(),
                None => panic!("All structure fields need to be identifiable")
            }
    }
}

pub fn find_str_attr_val<'a>(field: &'a Field, attr_name: &str) -> Option<&'a str> {
    for attr in field.attrs.iter() {
        match attr.value {
            MetaItem::NameValue(ref ident, ref it) => {
                if ident.as_ref() == attr_name {
                    return match it {
                        &Lit::Str(ref val, ..) => {
                            Some(val.as_ref())
                        },
                        _ => panic!("{} attribute must be a string", attr_name)
                    }
                }
            },
            _ => continue
        }
    }
    None
}

pub fn has_marker_attr(field :&Field, attr_name: &str) -> bool {
    for attr in field.attrs.iter() {
        match attr.value {
            MetaItem::Word(ref val) => if val == attr_name{
              return true;
            },
            _ => continue
        }
    }
    false
}

pub fn get_fields<'a>(ast: &'a DeriveInput, trait_name: &str) -> &'a Vec<Field> {
    let vd = match ast.body {
        Body::Enum(_) => panic ! ("{} can be only implemented for structures", trait_name),
        Body::Struct( ref val) => val
    };
    match vd {
        & VariantData::Struct( ref f) => f,
        & VariantData::Tuple(_) | &VariantData::Unit => panic ! ("{} can be only implemented for structures", trait_name)
    }
}