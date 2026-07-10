use crate::{enum_str::error_msg::ErrorMsgVar, gen_option::GenOption, template::TemplateSegment};

mod attr;
mod cases;
mod error_msg;
pub(super) mod generate;
pub(super) mod parse;

pub struct Ir<'a> {
    ident: &'a syn::Ident,
    generics: &'a syn::Generics,
    variants: Vec<VariantIr<'a>>,
    error: ErrorIr,
    gen_options: GenOptions,
}

pub struct GenOptions {
    fn_as_name: Option<GenOption>,
    fn_as_aliases: Option<GenOption>,
    const_all_names: Option<GenOption>,
    const_all_aliases: Option<GenOption>,
    impl_into_static_str: bool,
    impl_as_ref_str: bool,
    impl_display: bool,
    impl_from_str: bool,
    impl_try_from_str: bool,
}

#[derive(Clone)]
pub struct ErrorIr {
    ident: syn::Ident,
    vis: syn::Visibility,
    gen_error_struct: bool,
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
