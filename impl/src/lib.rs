extern crate proc_macro;

mod displayer;
mod fields;

use crate::displayer::Displayer;
use crate::fields::StructFields;
use displayer::DisplayArgs;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// Macro for deriving the [std::fmt::Display] Trait. It uses a Syntax similar to the [format!] Macro.
///
/// # Examples
///
/// ```
/// #[display("{} is a Person and is {} years old", name, age)]
/// struct Person {
///     pub name: &'static str,
///     pub age: u8,
/// }
/// assert_eq!(
///     "Person 3 is called Bob",
///     Person { id: 3, name: "Bob" }.to_string()
/// );
///
/// ```
#[proc_macro_attribute]
pub fn display(args: TokenStream, input: TokenStream) -> TokenStream {
    let type_definition = TokenStream2::from(input.clone());
    let input = parse_macro_input!(input as ItemStruct);
    let type_name = input.ident;
    let generics = input.generics;
    let where_clause = generics.where_clause.clone();
    let fields = StructFields::from(input.fields);

    //let args = parse_macro_input!(args as AttributeArgs);
    let parser = DisplayArgs::get_parser(fields.is_tuple_struct());
    let args = parse_macro_input!(args with parser);

    let displayer = match Displayer::new(args, fields) {
        Ok(displayer) => displayer,
        Err(err) => return err.to_compile_error().into(),
    };

    quote! {
        #type_definition
        #[allow(unused_qualifications)]
        impl #generics std::fmt::Display for #type_name #generics #where_clause {
            fn fmt(&self, __formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #displayer
            }
        }
    }
    .into()
}
