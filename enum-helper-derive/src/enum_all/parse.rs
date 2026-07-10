use proc_macro2::Span;
use quote::format_ident;

use crate::{
    ctxt::Ctxt,
    enum_all::{
        Ir, VariantIr,
        attr::{EnumAttr, VariantAttr},
    },
    gen_option::DefaultGenOption,
};

use super::GenOptions;

pub fn parse_ir(input: &syn::DeriveInput) -> syn::Result<Ir<'_>> {
    let syn::Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "EnumAll only supports enum",
        ));
    };

    let cx = Ctxt::new();

    let enum_attr = EnumAttr::from_attrs(&cx, &input.attrs);

    let mut variants = Vec::new();
    let mut is_unit_enum = true;

    for variant in &data_enum.variants {
        let v_ir = parse_variant_ir(&cx, variant);

        let is_unit = matches!(v_ir.fields, syn::Fields::Unit);

        if !v_ir.skip && !is_unit {
            is_unit_enum = false;
        }
        variants.push(v_ir);
    }

    let all_enabled = enum_attr.all.enabled_or(true);

    if !is_unit_enum && all_enabled {
        let e = syn::Error::new(
            Span::call_site(),
            "EnumAll cannot build `ALL` for non-unit variants; consider `#[enum_all(all(disable))]` to derive only `COUNT`, or `#[enum_all(skip)]` to skip them",
        );
        cx.syn_error(e);
    };

    let gen_options = make_gen_options(input, enum_attr);

    cx.check()?;

    Ok(Ir {
        ident: &input.ident,
        generics: &input.generics,
        variants,
        gen_options,
    })
}

fn parse_variant_ir(cx: &Ctxt, variant: &syn::Variant) -> VariantIr {
    let ident = variant.ident.clone();

    let attr = VariantAttr::from_attrs(cx, &variant.attrs);

    VariantIr {
        ident,
        fields: variant.fields.clone(),
        skip: attr.skip.get(),
    }
}

fn make_gen_options(input: &syn::DeriveInput, enum_attr: EnumAttr) -> GenOptions {
    let opt = |ident: &str| DefaultGenOption {
        enabled: true,
        ident: format_ident!("{}", ident),
        vis: input.vis.clone(),
    };

    let const_all = enum_attr.all.into_gen_option(opt("ALL"));
    let const_count = enum_attr.count.into_gen_option(opt("COUNT"));

    GenOptions {
        const_all,
        const_count,
    }
}
