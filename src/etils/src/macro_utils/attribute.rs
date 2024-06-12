use std::borrow::Borrow;
use syn::{Attribute, Field, Fields};
use crate::macro_utils::{Named, SimpleNamedAttributed};

pub fn get_attribute<'a, 'b, I: ?Sized + AsRef<str> + 'b, F: Borrow<Field> + 'a>(
    field: &'a F,
    attribute_ident: &'b I
) -> Option<&'a Attribute> {
    field.borrow()
        .attrs
        .iter()
        .find(| &attribute |
            attribute.path().is_ident(&attribute_ident)
        )
}

pub fn identify_attributed_field<
    'a,
    'b,
    I: ?Sized + AsRef<str>, >
(fields: &'a Fields, attribute_ident: &'b I) -> Option<SimpleNamedAttributed<&'a Field, &'a Attribute, String>> {
    identify_field(
        fields,
        | _, field |
            get_attribute(field, attribute_ident),
        | field, attribute, name |
            SimpleNamedAttributed::create(field, attribute, name)
    )

}

pub fn identify_field<
    'a,
    M: FnMut(&'a Field, E, String) -> T,
    P: FnMut(usize, &'a Field) -> Option<E>,
    T: Named<String>,
    E
>
(
    fields: &'a Fields,
    mut predicate: P,
    mut mapper: M
) -> Option<T> {
    fields
        .iter()
        .enumerate()
        .find_map(|(idx, field) |
            match predicate(idx, field) {
                None => None,
                Some(extra) => {
                    let name = field.borrow().ident.as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| idx.to_string());

                    return Some(mapper(field, extra, name))
                }
            }
        )
}