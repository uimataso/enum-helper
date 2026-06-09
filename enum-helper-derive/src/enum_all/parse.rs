use proc_macro2::Span;

use crate::{
    ctxt::Ctxt,
    enum_all::{Ir, VariantIr, attr::VariantAttr},
};

pub fn parse_ir(input: &syn::DeriveInput) -> syn::Result<Ir<'_>> {
    let syn::Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "EnumAll only supports enum",
        ));
    };

    let cx = Ctxt::new();

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

    if !is_unit_enum {
        let e = syn::Error::new(
            Span::call_site(),
            "EnumAll only supports unit enum. Consider using `#[enum_all(skip)]` to skip non unit variant",
        );
        cx.syn_error(e);
    };

    cx.check()?;

    Ok(Ir {
        ident: &input.ident,
        generics: &input.generics,
        variants,
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
