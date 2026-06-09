mod attr;
pub(super) mod generate;
pub(super) mod parse;

pub struct Ir<'a> {
    ident: &'a syn::Ident,
    generics: &'a syn::Generics,
    variants: Vec<VariantIr>,
}

pub struct VariantIr {
    ident: syn::Ident,
    fields: syn::Fields,
    skip: bool,
}
