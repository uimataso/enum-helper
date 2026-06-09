use quote::ToTokens as _;

use crate::{
    attr::{Attr as _, AttrBool, AttrVal},
    ctxt::Ctxt,
};

pub struct EnumAttr {
    pub attr: Vec<syn::Meta>,
    pub name: AttrVal<syn::Ident>,
    pub no_default_derive: AttrBool,
}

pub struct VariantAttr {
    pub attr: Vec<syn::Meta>,
    pub rename: AttrVal<syn::Ident>,
}

impl EnumAttr {
    pub fn from_attrs(cx: &Ctxt, attrs: &[syn::Attribute]) -> Self {
        use crate::symbol::*;

        let mut ret = Self {
            attr: Vec::new(),
            name: AttrVal::new(ENUM_KIND, NAME),
            no_default_derive: AttrBool::new(ENUM_KIND, NO_DEFAULT_DERIVE),
        };

        for attr in attrs {
            if attr.path() != ENUM_KIND {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            let res = attr.parse_nested_meta(|meta| {
                // special handle for #[enum_kind(attr(...))]
                if meta.path == ATTR {
                    match parse_attr(meta) {
                        Ok(m) => ret.attr.push(m),
                        Err(e) => cx.syn_error(e),
                    };
                } else if meta.path == ret.name.name() {
                    let p = |i: syn::Ident| Result::<_, &str>::Ok(i);
                    ret.name.try_from_meta(cx, &meta, p);
                } else if meta.path == ret.no_default_derive.name() {
                    ret.no_default_derive.try_from_meta(cx, &meta);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    let msg = format!("unknown {} container attribute `{}`", ENUM_KIND, path);
                    return Err(meta.error(msg));
                };

                Ok(())
            });

            if let Err(err) = res {
                cx.syn_error(err);
            }
        }

        ret
    }
}

impl VariantAttr {
    pub fn from_attrs(cx: &Ctxt, attrs: &[syn::Attribute]) -> Self {
        use crate::symbol::*;

        let mut ret = Self {
            attr: Vec::new(),
            rename: AttrVal::new(ENUM_KIND, RENAME),
        };

        for attr in attrs {
            if attr.path() != ENUM_KIND {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            let res = attr.parse_nested_meta(|meta| {
                // special handle for #[enum_kind(attr(...))]
                if meta.path == ATTR {
                    match parse_attr(meta) {
                        Ok(m) => ret.attr.push(m),
                        Err(e) => cx.syn_error(e),
                    };
                } else if meta.path == ret.rename.name() {
                    let p = |i: syn::Ident| Result::<_, &str>::Ok(i);
                    ret.rename.try_from_meta(cx, &meta, p);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    let msg = format!("unknown {} variant attribute `{}`", ENUM_KIND, path);
                    return Err(meta.error(msg));
                };

                Ok(())
            });

            if let Err(err) = res {
                cx.syn_error(err);
            }
        }

        ret
    }
}

fn parse_attr(meta: syn::meta::ParseNestedMeta<'_>) -> syn::Result<syn::Meta> {
    let content;
    syn::parenthesized!(content in meta.input);
    content.parse()
}
