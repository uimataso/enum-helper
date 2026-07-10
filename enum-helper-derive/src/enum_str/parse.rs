use std::collections::HashMap;

use proc_macro2::Span;
use quote::format_ident;

use crate::{
    ctxt::Ctxt,
    enum_str::{
        ErrorIr, Ir, VariantIr,
        attr::{EnumAttr, VariantAttr},
        cases::RenameRule,
        error_msg::{ErrorMsgVar, default_error_msg},
    },
    gen_option::DefaultGenOption,
    template::TemplateSegment,
};

use super::GenOptions;

pub fn parse_ir(input: &syn::DeriveInput) -> syn::Result<Ir<'_>> {
    let syn::Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "EnumStr only supports enum",
        ));
    };

    let cx = Ctxt::new();

    let enum_attr = EnumAttr::from_attrs(&cx, &input.attrs);

    let mut variants = Vec::new();
    let mut has_non_skip_non_unit_variant = false;
    for variant in &data_enum.variants {
        let v_ir = parse_variant_ir(&cx, variant, &enum_attr);

        if !(matches!(v_ir.fields, syn::Fields::Unit) || v_ir.skip) {
            has_non_skip_non_unit_variant = true;
        }

        variants.push(v_ir);
    }

    if !enum_attr.default.get() && has_non_skip_non_unit_variant {
        cx.syn_error(syn::Error::new(
            Span::call_site(),
            "EnumStr only supports unit enums by default; use #[enum_str(default)] or #[enum_str(skip)] to opt in to non-unit enum support, or use EnumKind to generate a unit enum",
        ));
    };

    if variants.is_empty() {
        let e = syn::Error::new(Span::call_site(), "EnumStr requires at least one variant");
        cx.syn_error(e);
    }

    check_name_ambiguous(&cx, &variants);

    let error = make_error_ir(input, &enum_attr);
    let gen_options = make_gen_options(input, enum_attr);

    cx.check()?;

    Ok(Ir {
        ident: &input.ident,
        generics: &input.generics,
        variants,
        error,
        gen_options,
    })
}

fn parse_variant_ir<'a>(
    cx: &Ctxt,
    variant: &'a syn::Variant,
    enum_attr: &EnumAttr,
) -> VariantIr<'a> {
    let ident = &variant.ident;
    let ident_str = ident.to_string();

    let attr = VariantAttr::from_attrs(cx, &variant.attrs);

    let name = {
        let rename_rule = enum_attr.rename_all.get().unwrap_or(&RenameRule::None);
        if let Some(rename) = attr.rename.get() {
            rename.value()
        } else {
            rename_rule.apply_to_variant(&ident_str)
        }
    };

    let aliases = {
        let mut ret = vec![name.clone()];
        for alias_rule in enum_attr.alias_all.get() {
            let alias = alias_rule.apply_to_variant(&ident_str);
            if !ret.contains(&alias) {
                ret.push(alias);
            }
        }
        for alias in attr.alias.get() {
            let alias = alias.value();
            if !ret.contains(&alias) {
                ret.push(alias);
            }
        }
        ret
    };

    VariantIr {
        ident,
        name,
        aliases,
        fields: &variant.fields,
        skip: attr.skip.get(),
    }
}

fn check_name_ambiguous(cx: &Ctxt, variants: &[VariantIr]) {
    let mut checked: HashMap<&String, &VariantIr> = Default::default();

    for variant in variants {
        // since aliases already contains `name`, we only need to check this.
        for name in &variant.aliases {
            if let Some(v) = checked.get(&name) {
                let msg = format!(
                    "ambiguous name `{}` between `{}` and `{}`",
                    name, v.ident, variant.ident
                );
                cx.error_spanned_by(variant.ident, msg);
                continue;
            }

            checked.insert(name, variant);
        }
    }
}

fn make_error_ir(input: &syn::DeriveInput, enum_attr: &EnumAttr) -> ErrorIr {
    let gen_error_struct = enum_attr.error.enabled_or(!enum_attr.no_parsing.get());

    let error_ident = if let Some(name) = enum_attr.error.name_val() {
        name.clone()
    } else {
        format_ident!("Invalid{}", &input.ident)
    };

    let segs = match enum_attr.error_msg.get() {
        Some(segs) => segs.clone(),
        None => default_error_msg(),
    };

    let should_store_input = segs
        .iter()
        .any(|s| matches!(s, TemplateSegment::Var(ErrorMsgVar::Input)));

    ErrorIr {
        ident: error_ident,
        vis: enum_attr.error.vis_val().unwrap_or(&input.vis).clone(),
        gen_error_struct,
        should_store_input,
        error_template: segs,
    }
}

fn make_gen_options(input: &syn::DeriveInput, enum_attr: EnumAttr) -> GenOptions {
    let rendering = !enum_attr.no_rendering.get();
    let parsing = !enum_attr.no_parsing.get();

    let r_opt = |ident: &str| DefaultGenOption {
        enabled: rendering,
        ident: format_ident!("{}", ident),
        vis: input.vis.clone(),
    };

    let fn_as_name = enum_attr.as_name.into_gen_option(r_opt("as_name"));
    let fn_as_aliases = enum_attr.as_aliases.into_gen_option(r_opt("as_aliases"));
    let const_all_names = enum_attr.all_names.into_gen_option(r_opt("ALL_NAMES"));
    let const_all_aliases = enum_attr.all_aliases.into_gen_option(r_opt("ALL_ALIASES"));

    let impl_into_static_str = enum_attr.impl_into_static_str.enabled_or(rendering);
    let impl_as_ref_str = enum_attr.impl_as_ref_str.enabled_or(rendering);

    let impl_from_str = enum_attr.impl_from_str.enabled_or(parsing);
    let impl_try_from_str = enum_attr.impl_try_from_str.enabled_or(parsing);

    GenOptions {
        fn_as_name,
        fn_as_aliases,
        const_all_names,
        const_all_aliases,
        impl_into_static_str,
        impl_as_ref_str,
        impl_from_str,
        impl_try_from_str,
    }
}
