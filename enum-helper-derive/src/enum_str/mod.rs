use crate::{enum_str::error_msg::ErrorMsgVar, template::TemplateSegment};

mod attr;
mod cases;
mod error_msg;
pub(super) mod generate;
pub(super) mod parse;

pub struct Ir<'a> {
    ident: &'a syn::Ident,
    vis: &'a syn::Visibility,
    generics: &'a syn::Generics,
    error: ErrorIr,
    variants: Vec<VariantIr<'a>>,

    gen_rendering: bool,
    gen_parsing: bool,
    gen_error_struct: bool,
}

pub struct ErrorIr {
    ident: syn::Ident,
    should_store_input: bool,
    error_template: Vec<TemplateSegment<ErrorMsgVar>>,
}

pub struct VariantIr<'a> {
    ident: &'a syn::Ident,
    name: String,
    /// include `name`
    aliases: Vec<String>,
    fields: &'a syn::Fields,
    skip: bool,
}
