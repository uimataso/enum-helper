use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    enum_str::{Ir, error_msg::ErrorMsgVar},
    gen_option::GenOption,
    template::TemplateSegment,
};

pub fn generate(ir: Ir<'_>) -> TokenStream {
    let mut blocks = Vec::new();

    if ir.gen_options.fn_as_name.is_some()
        || ir.gen_options.impl_into_static_str
        || ir.gen_options.impl_as_ref_str
        || ir.gen_options.impl_display
    {
        blocks.push(gen_fn_inner_as_name(&ir));
    }
    if ir.gen_options.impl_from_str || ir.gen_options.impl_try_from_str {
        blocks.push(gen_fn_inner_from_str(&ir));
    }

    if let Some(opt) = ir.gen_options.fn_as_name.clone() {
        blocks.push(gen_fn_as_name(&ir, opt));
    }
    if let Some(opt) = ir.gen_options.fn_as_aliases.clone() {
        blocks.push(gen_fn_as_aliases(&ir, opt));
    }
    if let Some(opt) = ir.gen_options.const_all_names.clone() {
        blocks.push(gen_const_all_names(&ir, opt));
    }
    if let Some(opt) = ir.gen_options.const_all_aliases.clone() {
        blocks.push(gen_const_all_aliases(&ir, opt));
    }
    if ir.gen_options.impl_into_static_str {
        blocks.push(gen_impl_into_static_str(&ir));
    }
    if ir.gen_options.impl_as_ref_str {
        blocks.push(gen_impl_as_ref_str(&ir));
    }
    if ir.gen_options.impl_display {
        blocks.push(gen_impl_display(&ir));
    }

    if ir.error.gen_error_struct {
        blocks.push(gen_error_struct(&ir));
        blocks.push(gen_error_impl_display(&ir));
        blocks.push(gen_error_impl_error(&ir));
    }

    if ir.gen_options.impl_from_str {
        blocks.push(gen_impl_from_str(&ir));
    }
    if ir.gen_options.impl_try_from_str {
        blocks.push(gen_impl_try_from_str(&ir));
    }

    quote! {
        #(#blocks)*
    }
}

fn gen_fn_inner_as_name(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let as_name_arms: Vec<_> = ir
        .variants
        .iter()
        .map(|v| {
            let v_ident = &v.ident;
            let name = &v.name;
            let pat = match v.fields {
                syn::Fields::Named(_) => quote! { Self::#v_ident {..} },
                syn::Fields::Unnamed(_) => quote! { Self::#v_ident(..) },
                syn::Fields::Unit => quote! { Self::#v_ident },
            };
            quote! { #pat => #name  }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            const fn __enum_str_as_name(&self) -> &'static str {
                match self {
                    #(#as_name_arms,)*
                }
            }
        }
    }
}

fn gen_fn_inner_from_str(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();
    let error_ident = &ir.error.ident;

    let default_val = quote! { ::core::default::Default::default() };

    let arms: Vec<_> = ir
        .variants
        .iter()
        .filter(|v| !v.skip)
        .map(|v| {
            let v_ident = &v.ident;
            let aliases = &v.aliases;
            let pat = quote! { #(#aliases)|* };
            let val = match v.fields {
                syn::Fields::Named(f) => {
                    let fields: Vec<_> = f
                        .named
                        .iter()
                        .map(|x| {
                            let i = x.ident.clone().expect("named field should have ident");
                            quote! { #i: #default_val }
                        })
                        .collect();
                    quote! { Self::#v_ident { #(#fields),* } }
                }
                syn::Fields::Unnamed(f) => {
                    let fields = std::iter::repeat_n(default_val.clone(), f.unnamed.len());
                    quote! { Self::#v_ident(#(#fields),*) }
                }
                syn::Fields::Unit => quote! { Self::#v_ident },
            };
            quote! { #pat => ::core::result::Result::Ok(#val) }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            fn __enum_str_from_str(s: &str) -> ::core::result::Result<Self, #error_ident> {
                match s {
                    #(#arms,)*
                    _ => ::core::result::Result::Err(#error_ident::new(s)),
                }
            }
        }
    }
}

fn gen_fn_as_name(ir: &Ir<'_>, opt: GenOption) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let fn_ident = opt.ident;
    let fn_vis = opt.vis;

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #fn_vis const fn #fn_ident(&self) -> &'static str {
                self.__enum_str_as_name()
            }
        }
    }
}

fn gen_fn_as_aliases(ir: &Ir<'_>, opt: GenOption) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let fn_ident = opt.ident;
    let fn_vis = opt.vis;

    let as_aliases_arms: Vec<_> = ir
        .variants
        .iter()
        .map(|v| {
            let v_ident = &v.ident;
            let alias = &v.aliases;
            let pat = match v.fields {
                syn::Fields::Named(_) => quote! { Self::#v_ident {..} },
                syn::Fields::Unnamed(_) => quote! { Self::#v_ident(..) },
                syn::Fields::Unit => quote! { Self::#v_ident },
            };
            quote! { #pat => &[#(#alias),*] }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #fn_vis const fn #fn_ident(&self) -> &'static [&'static str] {
                match self {
                    #(#as_aliases_arms,)*
                }
            }
        }
    }
}

fn gen_const_all_names(ir: &Ir<'_>, opt: GenOption) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let const_ident = opt.ident;
    let const_vis = opt.vis;

    let arr = ir
        .variants
        .iter()
        .filter(|v| !v.skip)
        .map(|v| v.name.as_str())
        .collect::<Vec<_>>();
    let len = arr.len();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #const_vis const #const_ident: [&'static str; #len] = [#(#arr),*];
        }
    }
}

fn gen_const_all_aliases(ir: &Ir<'_>, opt: GenOption) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    let const_ident = opt.ident;
    let const_vis = opt.vis;

    let arr = ir
        .variants
        .iter()
        .filter(|v| !v.skip)
        .flat_map(|v| v.aliases.iter().map(|s| s.as_str()))
        .collect::<Vec<_>>();
    let len = arr.len();

    quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #const_vis const #const_ident: [&'static str; #len] = [#(#arr),*];
        }
    }
}

fn gen_impl_into_static_str(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::core::convert::From<#ident #ty_generics> for &'static str #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                value.__enum_str_as_name()
            }
        }
    }
}

fn gen_impl_as_ref_str(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::core::convert::AsRef<str> for #ident #ty_generics #where_clause {
            fn as_ref(&self) -> &str {
                self.__enum_str_as_name()
            }
        }
    }
}

fn gen_impl_display(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(self.__enum_str_as_name(), f)
            }
        }
    }
}

fn gen_error_struct(ir: &Ir) -> TokenStream {
    let error_vis = &ir.error.vis;
    let error_ident = &ir.error.ident;

    if ir.error.should_store_input {
        quote! {
            #[automatically_derived]
            #[non_exhaustive]
            #[derive(Debug, Clone)]
            #error_vis struct #error_ident {
                pub input: ::std::string::String,
            }

            #[automatically_derived]
            impl #error_ident {
                pub fn new(input: &str) -> Self {
                    Self {
                        input: input.to_string(),
                    }
                }
            }
        }
    } else {
        quote! {
            #[automatically_derived]
            #[non_exhaustive]
            #[derive(Debug, Clone)]
            #error_vis struct #error_ident {}

            #[automatically_derived]
            impl #error_ident {
                pub fn new(_input: &str) -> Self {
                    Self {}
                }
            }
        }
    }
}

fn gen_error_impl_display(ir: &Ir<'_>) -> TokenStream {
    let error_ident = &ir.error.ident;

    fn seg_to_arg(var: &TemplateSegment<ErrorMsgVar>, ir: &Ir<'_>) -> TokenStream {
        match var {
            TemplateSegment::Lit(s) => quote! { #s },
            TemplateSegment::Var(var) => match var {
                ErrorMsgVar::Name => {
                    let s = ir.ident.to_string();
                    quote! { #s }
                }
                ErrorMsgVar::Names(list_mod) => {
                    let (sep, quote) = list_mod.get();
                    let names = ir
                        .variants
                        .iter()
                        .filter(|v| !v.skip)
                        .map(|v| format!("{}{}{}", quote, v.name, quote))
                        .collect::<Vec<_>>()
                        .join(sep);
                    quote! { #names }
                }
                ErrorMsgVar::Aliases(list_mod) => {
                    let (sep, quote) = list_mod.get();
                    let aliases = ir
                        .variants
                        .iter()
                        .filter(|v| !v.skip)
                        .flat_map(|v| v.aliases.iter().map(|a| format!("{}{}{}", quote, a, quote)))
                        .collect::<Vec<_>>()
                        .join(sep);
                    quote! { #aliases }
                }
                ErrorMsgVar::Input => {
                    quote! { &self.input }
                }
            },
        }
    }

    let format_str: String = std::iter::repeat_n("{}", ir.error.error_template.len()).collect();

    let args: Vec<_> = ir
        .error
        .error_template
        .iter()
        .map(|s| seg_to_arg(s, ir))
        .collect();

    quote! {
        #[automatically_derived]
        impl ::core::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, #format_str #(, #args)*)
            }
        }
    }
}

fn gen_error_impl_error(ir: &Ir<'_>) -> TokenStream {
    let error_ident = &ir.error.ident;

    quote! {
        #[automatically_derived]
        impl ::core::error::Error for #error_ident { }
    }
}

fn gen_impl_from_str(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();
    let error_ident = &ir.error.ident;

    quote! {
        #[automatically_derived]
        impl #impl_generics ::core::str::FromStr for #ident #ty_generics #where_clause {
            type Err = #error_ident;

            fn from_str(s: &str) -> ::core::result::Result<Self, <Self as ::core::str::FromStr>::Err> {
                Self::__enum_str_from_str(s)
            }
        }
    }
}

fn gen_impl_try_from_str(ir: &Ir<'_>) -> TokenStream {
    let ident = &ir.ident;
    let (impl_generics, ty_generics, where_clause) = &ir.generics.split_for_impl();
    let error_ident = &ir.error.ident;

    quote! {
        #[automatically_derived]
        impl #impl_generics ::core::convert::TryFrom<&str> for #ident #ty_generics #where_clause {
            type Error = #error_ident;

            fn try_from(value: &str) -> ::core::result::Result<Self, <Self as ::core::convert::TryFrom<&str>>::Error> {
                Self::__enum_str_from_str(value)
            }
        }
    }
}
