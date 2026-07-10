use proc_macro2::TokenStream;
use quote::quote;

use crate::enum_kind::Ir;

pub fn generate(ir: Ir<'_>) -> TokenStream {
    let mut blocks = Vec::new();

    blocks.push(gen_kind_struct(&ir));
    blocks.push(gen_fn_kind(&ir));

    quote! {
        #(#blocks)*
    }
}

fn gen_kind_struct(ir: &Ir<'_>) -> TokenStream {
    let vis = ir.vis;
    let kind_ident = &ir.kind_ident;

    let attrs = &ir.attrs;

    let default_derive = if ir.default_derive {
        quote! {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
            )]
        }
    } else {
        quote! {}
    };

    let variants: Vec<_> = ir
        .variants
        .iter()
        .map(|v| {
            let v_name = &v.kind_name;
            let attrs = &v.attrs;
            quote! {
                #(#[#attrs])*
                #v_name,
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        #default_derive
        #(#[#attrs])*
        #vis enum #kind_ident {
            #(#variants)*
        }
    }
}

fn gen_fn_kind(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let vis = &ir.vis;
    let kind_ident = &ir.kind_ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let arms: Vec<_> = ir
        .variants
        .iter()
        .map(|v| {
            let v_ident = &v.ident;
            let kind_name = &v.kind_name;
            let lhs = match v.fields {
                syn::Fields::Named(_) => quote! { Self::#v_ident {..} },
                syn::Fields::Unnamed(_) => quote! { Self::#v_ident(..) },
                syn::Fields::Unit => quote! { Self::#v_ident },
            };
            quote! { #lhs => #kind_ident::#kind_name }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #vis const fn kind(&self) -> #kind_ident {
                match self {
                    #(#arms,)*
                }
            }
        }
    }
}
