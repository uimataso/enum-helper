use quote::ToTokens as _;

use crate::{
    attr::{self, Attr as _, AttrBool, AttrVal, AttrVec, DeriveAttr, ImplAttr},
    ctxt::Ctxt,
    enum_str::{cases::RenameRule, error_msg::ErrorMsgVar},
    template::{TemplateSegment, parse_template},
};

pub struct EnumAttr {
    pub rename_all: AttrVal<RenameRule>,
    pub alias_all: AttrVec<RenameRule>,
    pub default: AttrBool,

    pub as_name: DeriveAttr,
    pub as_aliases: DeriveAttr,
    pub all_names: DeriveAttr,
    pub all_aliases: DeriveAttr,

    pub impl_into_static_str: ImplAttr,
    pub impl_as_ref_str: ImplAttr,
    pub impl_display: ImplAttr,
    pub impl_from_str: ImplAttr,
    pub impl_try_from_str: ImplAttr,

    pub error: DeriveAttr,
    pub error_msg: AttrVal<Vec<TemplateSegment<ErrorMsgVar>>>,

    pub no_rendering: AttrBool,
    pub no_parsing: AttrBool,
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

            as_name: DeriveAttr::new(ENUM_STR, AS_NAME),
            as_aliases: DeriveAttr::new(ENUM_STR, AS_ALIASES),
            all_names: DeriveAttr::new(ENUM_STR, ALL_NAMES),
            all_aliases: DeriveAttr::new(ENUM_STR, ALL_ALIASES),

            impl_into_static_str: ImplAttr::new(ENUM_STR, IMPL_INTO_STATIC_STR),
            impl_as_ref_str: ImplAttr::new(ENUM_STR, IMPL_AS_REF_STR),
            impl_display: ImplAttr::new(ENUM_STR, IMPL_DISPLAY),
            impl_from_str: ImplAttr::new(ENUM_STR, IMPL_FROM_STR),
            impl_try_from_str: ImplAttr::new(ENUM_STR, IMPL_TRY_FROM_STR),

            error: DeriveAttr::new(ENUM_STR, ERROR),
            error_msg: AttrVal::new(ENUM_STR, ERROR_MSG),

            no_rendering: AttrBool::new(ENUM_STR, NO_RENDERING),
            no_parsing: AttrBool::new(ENUM_STR, NO_PARSING),
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
                    ret.rename_all.try_from_meta_map(cx, &meta, p);
                } else if meta.path == ret.alias_all.name() {
                    let p = |s: syn::LitStr| RenameRule::from_str(&s.value());
                    ret.alias_all.try_from_meta_map(cx, &meta, p);
                } else if meta.path == ret.default.name() {
                    ret.default.try_from_meta(cx, &meta);
                } else if meta.path == ret.as_name.name() {
                    ret.as_name.try_from_meta(cx, &meta)
                } else if meta.path == ret.as_aliases.name() {
                    ret.as_aliases.try_from_meta(cx, &meta)
                } else if meta.path == ret.all_names.name() {
                    ret.all_names.try_from_meta(cx, &meta)
                } else if meta.path == ret.all_aliases.name() {
                    ret.all_aliases.try_from_meta(cx, &meta)
                } else if meta.path == ret.impl_into_static_str.name() {
                    ret.impl_into_static_str.try_from_meta(cx, &meta)
                } else if meta.path == ret.impl_as_ref_str.name() {
                    ret.impl_as_ref_str.try_from_meta(cx, &meta)
                } else if meta.path == ret.impl_display.name() {
                    ret.impl_display.try_from_meta(cx, &meta)
                } else if meta.path == ret.impl_from_str.name() {
                    ret.impl_from_str.try_from_meta(cx, &meta)
                } else if meta.path == ret.impl_try_from_str.name() {
                    ret.impl_try_from_str.try_from_meta(cx, &meta)
                } else if meta.path == ret.error.name() {
                    ret.error.try_from_meta(cx, &meta)
                } else if meta.path == ret.error_msg.name() {
                    let p = |s: syn::LitStr| parse_template(&s.value());
                    ret.error_msg.try_from_meta_map(cx, &meta, p);
                } else if meta.path == ret.no_rendering.name() {
                    ret.no_rendering.try_from_meta(cx, &meta);
                } else if meta.path == ret.no_parsing.name() {
                    ret.no_parsing.try_from_meta(cx, &meta);
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

        attr::check_conflict(cx, &ret.no_rendering, &ret.no_parsing);

        if !ret.error.enabled_or(!ret.no_parsing.get()) {
            if let Some(tok) = ret.error_msg.path_token() {
                cx.error_spanned_by(
                    &tok,
                    "`error_msg` has no effect when the error struct is not generated (disabled via `error(disable)` or `no_parsing`)",
                );
            }
        }

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
                    ret.rename.try_from_meta(cx, &meta);
                } else if meta.path == ret.alias.name() {
                    ret.alias.try_from_meta(cx, &meta);
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
