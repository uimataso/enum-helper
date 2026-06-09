use proc_macro2::TokenStream;
use quote::quote;

use crate::enum_all::Ir;

pub fn generate(ir: Ir<'_>) -> TokenStream {
    gen_impl_enum_all(&ir)
}

fn gen_impl_enum_all(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let all_array: Vec<_> = ir
        .variants
        .iter()
        .filter(|v| !v.skip)
        .map(|v| {
            let v_ident = &v.ident;
            quote! { Self::#v_ident }
        })
        .collect();

    let count = all_array.len();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::enum_helper::EnumAll for #ident #ty_generics #where_clause {
            type All = [Self; #count];
            const ALL: Self::All = [#(#all_array,)*];
        }
    }
}
