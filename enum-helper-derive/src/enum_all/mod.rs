mod attr;
pub(super) mod generate;
pub(super) mod parse;

use crate::gen_option::GenOption;

pub struct Ir<'a> {
    ident: &'a syn::Ident,
    generics: &'a syn::Generics,
    variants: Vec<VariantIr>,
    gen_options: GenOptions,
}

pub struct GenOptions {
    const_all: Option<GenOption>,
    const_count: Option<GenOption>,
}

pub struct VariantIr {
    ident: syn::Ident,
    fields: syn::Fields,
    skip: bool,
}
