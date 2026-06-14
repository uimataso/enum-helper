use quote::ToTokens as _;

use crate::{
    attr::{self, Attr as _, AttrBool, AttrVal, AttrVec},
    ctxt::Ctxt,
    enum_str::{cases::RenameRule, error_msg::ErrorMsgVar},
    template::{TemplateSegment, parse_template},
};

pub struct EnumAttr {
    pub rename_all: AttrVal<RenameRule>,
    pub alias_all: AttrVec<RenameRule>,
    pub default: AttrBool,
    pub error_name: AttrVal<syn::Ident>,
    pub error_msg: AttrVal<Vec<TemplateSegment<ErrorMsgVar>>>,
    pub no_rendering: AttrBool,
    pub no_parsing: AttrBool,
    pub no_error_struct: AttrBool,
}

pub struct VariantAttr {
    pub rename: AttrVal<syn::LitStr>,
    pub alias: AttrVec<syn::LitStr>,
    pub skip: AttrBool,
}

impl EnumAttr {
    pub fn from_attrs(cx: &Ctxt, attrs: &[syn::Attribute]) -> Self {
        use crate::symbol::*;

        let mut ret = Self {
            rename_all: AttrVal::new(ENUM_STR, RENAME_ALL),
            alias_all: AttrVec::new(ENUM_STR, ALIAS_ALL),
            default: AttrBool::new(ENUM_STR, DEFAULT),
            error_name: AttrVal::new(ENUM_STR, ERROR_NAME),
            error_msg: AttrVal::new(ENUM_STR, ERROR_MSG),
            no_rendering: AttrBool::new(ENUM_STR, NO_RENDERING),
            no_parsing: AttrBool::new(ENUM_STR, NO_PARSING),
            no_error_struct: AttrBool::new(ENUM_STR, NO_ERROR_STRUCT),
        };

        for attr in attrs {
            if attr.path() != ENUM_STR {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            let res = attr.parse_nested_meta(|meta| {
                if meta.path == ret.rename_all.name() {
                    let p = |s: syn::LitStr| RenameRule::from_str(&s.value());
                    ret.rename_all.try_from_meta(cx, &meta, p);
                } else if meta.path == ret.alias_all.name() {
                    let p = |s: syn::LitStr| RenameRule::from_str(&s.value());
                    ret.alias_all.try_from_meta(cx, &meta, p);
                } else if meta.path == ret.default.name() {
                    ret.default.try_from_meta(cx, &meta);
                } else if meta.path == ret.error_name.name() {
                    let p = |i: syn::Ident| Result::<_, &str>::Ok(i);
                    ret.error_name.try_from_meta(cx, &meta, p);
                } else if meta.path == ret.error_msg.name() {
                    let p = |s: syn::LitStr| parse_template(&s.value());
                    ret.error_msg.try_from_meta(cx, &meta, p);
                } else if meta.path == ret.no_rendering.name() {
                    ret.no_rendering.try_from_meta(cx, &meta);
                } else if meta.path == ret.no_parsing.name() {
                    ret.no_parsing.try_from_meta(cx, &meta);
                } else if meta.path == ret.no_error_struct.name() {
                    ret.no_error_struct.try_from_meta(cx, &meta);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    let msg = format!("unknown {} container attribute `{}`", ENUM_STR, path);
                    return Err(meta.error(msg));
                };

                Ok(())
            });

            if let Err(err) = res {
                cx.syn_error(err);
            }
        }

        ret.alias_all.check_dup_val(cx);

        attr::check_conflict(cx, &ret.error_msg, &ret.no_error_struct);
        attr::check_conflict(cx, &ret.error_msg, &ret.no_parsing);

        attr::check_conflict(cx, &ret.no_rendering, &ret.no_parsing);

        ret
    }
}

impl VariantAttr {
    pub fn from_attrs(cx: &Ctxt, attrs: &[syn::Attribute]) -> Self {
        use crate::symbol::*;

        let mut ret = Self {
            rename: AttrVal::new(ENUM_STR, RENAME),
            alias: AttrVec::new(ENUM_STR, ALIAS),
            skip: AttrBool::new(ENUM_STR, SKIP),
        };

        for attr in attrs {
            if attr.path() != ENUM_STR {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            let res = attr.parse_nested_meta(|meta| {
                if meta.path == ret.rename.name() {
                    let p = |s: syn::LitStr| Result::<_, &str>::Ok(s);
                    ret.rename.try_from_meta(cx, &meta, p);
                } else if meta.path == ret.alias.name() {
                    let p = |s: syn::LitStr| Result::<_, &str>::Ok(s);
                    ret.alias.try_from_meta(cx, &meta, p);
                } else if meta.path == ret.skip.name() {
                    ret.skip.try_from_meta(cx, &meta);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    let msg = format!("unknown {} variant attribute `{}`", ENUM_STR, path);
                    return Err(meta.error(msg));
                };

                Ok(())
            });

            if let Err(err) = res {
                cx.syn_error(err);
            }
        }

        ret.alias.check_dup_val(cx);

        ret
    }
}
