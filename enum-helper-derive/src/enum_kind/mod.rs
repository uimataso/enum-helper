mod attr;
pub(super) mod generate;
pub(super) mod parse;

pub struct Ir<'a> {
    ident: &'a syn::Ident,
    vis: &'a syn::Visibility,
    generics: &'a syn::Generics,
    variants: Vec<VariantIr<'a>>,
    kind_ident: syn::Ident,
    attrs: Vec<syn::Meta>,
    default_derive: bool,
}

pub struct VariantIr<'a> {
    ident: &'a syn::Ident,
    fields: &'a syn::Fields,
    kind_name: syn::Ident,
    attrs: Vec<syn::Meta>,
}
