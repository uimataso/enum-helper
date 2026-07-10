use quote::ToTokens as _;

use crate::{
    attr::{Attr as _, AttrBool, DeriveAttr},
    ctxt::Ctxt,
};

pub struct EnumAttr {
    pub all: DeriveAttr,
    pub count: DeriveAttr,
}

impl EnumAttr {
    pub fn from_attrs(cx: &Ctxt, attrs: &[syn::Attribute]) -> Self {
        use crate::symbol::*;

        let mut ret = Self {
            all: DeriveAttr::new(ENUM_ALL, ALL),
            count: DeriveAttr::new(ENUM_ALL, COUNT),
        };

        for attr in attrs {
            if attr.path() != ENUM_ALL {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            let res = attr.parse_nested_meta(|meta| {
                if meta.path == ret.all.name() {
                    ret.all.try_from_meta(cx, &meta)
                } else if meta.path == ret.count.name() {
                    ret.count.try_from_meta(cx, &meta)
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    let msg = format!("unknown {} container attribute `{}`", ENUM_ALL, path);
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

pub struct VariantAttr {
    pub skip: AttrBool,
}

impl VariantAttr {
    pub fn from_attrs(cx: &Ctxt, attrs: &[syn::Attribute]) -> Self {
        use crate::symbol::*;

        let mut ret = Self {
            skip: AttrBool::new(ENUM_ALL, SKIP),
        };

        for attr in attrs {
            if attr.path() != ENUM_ALL {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            let res = attr.parse_nested_meta(|meta| {
                if meta.path == ret.skip.name() {
                    ret.skip.try_from_meta(cx, &meta);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    let msg = format!("unknown {} variant attribute `{}`", ENUM_ALL, path);
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
