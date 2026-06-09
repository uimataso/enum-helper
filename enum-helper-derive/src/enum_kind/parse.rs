use proc_macro2::Span;
use quote::format_ident;

use crate::{
    ctxt::Ctxt,
    enum_kind::{
        Ir, VariantIr,
        attr::{EnumAttr, VariantAttr},
    },
};

pub fn parse_ir(input: &syn::DeriveInput) -> syn::Result<Ir<'_>> {
    let syn::Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "EnumKind only supports enum",
        ));
    };

    let cx = Ctxt::new();

    let enum_attr = EnumAttr::from_attrs(&cx, &input.attrs);

    let mut variants = Vec::new();

    for variant in &data_enum.variants {
        let v_ir = parse_variant_ir(&cx, variant);
        variants.push(v_ir);
    }

    if variants.is_empty() {
        let e = syn::Error::new(Span::call_site(), "EnumKind requires at least one variant");
        cx.syn_error(e);
    }

    let ident = &input.ident;

    let kind_ident = match enum_attr.name.get() {
        Some(n) => n.clone(),
        None => format_ident!("{}Kind", ident),
    };

    cx.check()?;

    Ok(Ir {
        ident: &input.ident,
        vis: &input.vis,
        generics: &input.generics,
        variants,
        kind_ident,
        attrs: enum_attr.attr,
        default_derive: !enum_attr.no_default_derive.get(),
    })
}

fn parse_variant_ir<'a>(cx: &Ctxt, variant: &'a syn::Variant) -> VariantIr<'a> {
    let ident = &variant.ident;

    let attr = VariantAttr::from_attrs(cx, &variant.attrs);

    let kind_name = match attr.rename.get() {
        Some(n) => n.clone(),
        None => ident.clone(),
    };

    VariantIr {
        ident,
        fields: &variant.fields,
        kind_name,
        attrs: attr.attr,
    }
}
