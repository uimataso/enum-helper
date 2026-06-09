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
    template::TemplateSegment,
};

pub fn parse_ir(input: &syn::DeriveInput) -> syn::Result<Ir<'_>> {
    let syn::Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "EnumStr only supports unit enum",
        ));
    };

    let cx = Ctxt::new();

    let enum_attr = EnumAttr::from_attrs(&cx, &input.attrs);

    let mut variants = Vec::new();
    let mut is_unit_enum = true;
    for variant in &data_enum.variants {
        let v_ir = parse_variant_ir(&cx, variant, &enum_attr);
        if !v_ir.is_unit {
            is_unit_enum = false;
        }
        variants.push(v_ir);
    }

    if !is_unit_enum {
        cx.syn_error(syn::Error::new(
            Span::call_site(),
            "EnumStr only supports unit enum",
        ));
    };

    if variants.is_empty() {
        let e = syn::Error::new(Span::call_site(), "EnumStr requires at least one variant");
        cx.syn_error(e);
    }

    check_name_ambiguous(&cx, &variants);

    let error = make_error_ir(input, &enum_attr);

    cx.check()?;

    Ok(Ir {
        ident: &input.ident,
        vis: &input.vis,
        generics: &input.generics,
        error,
        variants,
        gen_rendering: !enum_attr.no_rendering.get(),
        gen_parsing: !enum_attr.no_parsing.get(),
        gen_error_struct: !(enum_attr.no_parsing.get() || enum_attr.no_error_struct.get()),
    })
}

fn parse_variant_ir(cx: &Ctxt, variant: &syn::Variant, enum_attr: &EnumAttr) -> VariantIr {
    let ident = variant.ident.clone();
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

    let is_unit = matches!(variant.fields, syn::Fields::Unit);

    VariantIr {
        ident,
        name,
        aliases,
        is_unit,
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
                cx.error_spanned_by(&variant.ident, msg);
                continue;
            }

            checked.insert(name, variant);
        }
    }
}

fn make_error_ir(input: &syn::DeriveInput, enum_attr: &EnumAttr) -> ErrorIr {
    let error_ident = if let Some(name) = enum_attr.error_name.get() {
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
        should_store_input,
        error_template: segs,
    }
}
