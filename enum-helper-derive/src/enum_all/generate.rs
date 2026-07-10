use proc_macro2::TokenStream;
use quote::quote;

use crate::{enum_all::Ir, gen_option::GenOption};

pub fn generate(ir: Ir<'_>) -> TokenStream {
    let mut blocks = Vec::new();

    if let Some(opt) = ir.gen_options.const_all.clone() {
        blocks.push(gen_const_all(&ir, opt));
    }
    if let Some(opt) = ir.gen_options.const_count.clone() {
        blocks.push(gen_const_count(&ir, opt));
    }

    quote! {
        #(#blocks)*
    }
}

fn gen_const_all(ir: &Ir<'_>, opt: GenOption) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let const_ident = opt.ident;
    let const_vis = opt.vis;

    let arr: Vec<_> = ir
        .variants
        .iter()
        .filter(|v| !v.skip)
        .map(|v| {
            let v_ident = &v.ident;
            quote! { Self::#v_ident }
        })
        .collect();

    let len = arr.len();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #const_vis const #const_ident: [Self; #len] = [#(#arr),*];
        }
    }
}

fn gen_const_count(ir: &Ir<'_>, opt: GenOption) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let const_ident = opt.ident;
    let const_vis = opt.vis;

    let count = ir.variants.iter().filter(|v| !v.skip).count();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #const_vis const #const_ident: usize = #count;
        }
    }
}
