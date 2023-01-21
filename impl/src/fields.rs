use quote::quote;
use quote::{format_ident, ToTokens};
use syn::Fields;

#[derive(Debug)]
pub(crate) enum StructFields {
    NormalStruct(Vec<proc_macro2::Ident>),
    TupleStruct(Vec<proc_macro2::Ident>),
    UnitStruct,
}

impl From<Fields> for StructFields {
    fn from(struct_fields: Fields) -> Self {
        match struct_fields {
            Fields::Named(named_fields) => Self::NormalStruct(
                named_fields
                    .named
                    .iter()
                    .map(|field| field.ident.clone().expect("field has ident"))
                    .collect(),
            ),
            Fields::Unnamed(unnamed_fields) => Self::TupleStruct(
                unnamed_fields
                    .unnamed
                    .iter()
                    .enumerate()
                    // _field is always None in a tuple struct
                    .map(|(index, _field)| format_ident!("_{}", index))
                    .collect(),
            ),
            Fields::Unit => Self::UnitStruct,
        }
    }
}

impl ToTokens for StructFields {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            StructFields::NormalStruct(fields) => quote! {
                { #(#fields),* }
            },
            StructFields::TupleStruct(fields) => quote! {
                ( #(#fields),* )
            },
            StructFields::UnitStruct => quote! {},
        });
    }
}

impl StructFields {
    pub fn is_tuple_struct(&self) -> bool {
        if let StructFields::TupleStruct(_) = self {
            true
        } else {
            false
        }
    }
}
