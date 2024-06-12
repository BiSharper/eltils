

extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{Data, DeriveInput, LitBool, parse_macro_input};
use syn::__private::quote::{quote_spanned};
use syn::parse::Parse;
use etils::macro_utils::{Attributed, identify_attributed_field, Named};

const DEREF_ATTRIBUTE: &str = "deref";


#[proc_macro_derive(Deref, attributes(deref))]
pub fn auto_deref_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("Deref only supports structs"),
    };
    let Some(deref_field) = identify_attributed_field(
        &data.fields,
        DEREF_ATTRIBUTE
    ) else {
        panic!("Could not locate position of deref field, be sure its annotated with `{}`", DEREF_ATTRIBUTE);
    };

    let mutable = deref_field
        .attribute()
        .parse_args_with(LitBool::parse).unwrap()
        .value;

    let deref_type = &deref_field.ty;
    let deref_name = &**deref_field.name();

    let mut  expanded = quote_spanned! { name.span() =>
        impl #impl_generics core::ops::deref::Deref for #name #type_generics #where_clause {
            type Target = #deref_type;

            fn deref(&self) -> &Self::Target { &self.#deref_name }
        }
    };
    if mutable {
        expanded.extend(quote_spanned! { name.span() =>
            impl #impl_generics core::ops::DerefMut for #name #type_generics #where_clause {
                fn deref_mut(&mut self) -> &mut Self::Target { &mut self.#deref_name }
            }
        });
    }

    TokenStream::from(expanded)
}