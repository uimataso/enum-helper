use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{meta::ParseNestedMeta, parse::Parse};

use crate::{
    ctxt::Ctxt,
    gen_option::{DefaultGenOption, GenOption},
    symbol::Symbol,
};

pub trait Attr {
    fn attr(&self) -> Symbol;
    fn name(&self) -> Symbol;
    fn is_set(&self) -> bool;
    fn path_token(&self) -> Option<TokenStream>;
}

pub struct AttrVal<T> {
    attr: Symbol,
    name: Symbol,

    path: Option<TokenStream>,
    input: Option<TokenStream>,
    value: Option<T>,
}

pub struct AttrBool {
    attr: Symbol,
    name: Symbol,
    path: Option<TokenStream>,
}

pub struct AttrVec<T> {
    attr: Symbol,
    name: Symbol,
    inner: Vec<AttrVecInner<T>>,
}

struct AttrVecInner<T> {
    path: TokenStream,
    input: TokenStream,
    value: T,
}

/// Parse attr for `xxx(name = xxx, vis = "pub", disabled)`
pub struct DeriveAttr {
    attr: Symbol,
    name: Symbol,
    name_val: Option<syn::Ident>,
    vis_val: Option<syn::Visibility>,
    enabled: Option<bool>,
    path: Option<TokenStream>,
}

/// Parse attr for `xxx(disabled)`
pub struct ImplAttr {
    attr: Symbol,
    name: Symbol,
    enabled: Option<bool>,
    path: Option<TokenStream>,
}

pub fn check_conflict(cx: &Ctxt, a: &impl Attr, b: &impl Attr) {
    if let (Some(_a_token), Some(b_token)) = (a.path_token(), b.path_token()) {
        let msg = format!(
            "{} attribute `{}` and `{}` conflicts with each other",
            a.attr(),
            a.name(),
            b.name()
        );
        cx.error_spanned_by(b_token, msg);
    }
}

impl<T> AttrVal<T> {
    pub fn new(attr: Symbol, name: Symbol) -> Self {
        Self {
            attr,
            name,
            path: None,
            input: None,
            value: None,
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn try_from_meta(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>)
    where
        T: Parse,
    {
        self.try_from_meta_map(cx, meta, |t| Result::<T, &str>::Ok(t))
    }

    pub fn try_from_meta_map<V, F, E>(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>, f: F)
    where
        V: Parse,
        F: Fn(V) -> Result<T, E>,
        E: core::fmt::Display,
    {
        // We need to parse the value for the error message to be correct.
        if self.is_set() {
            let msg = format!("duplicate {} attribute `{}`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);
        }

        match parse_meta(meta, f) {
            Ok((val, token)) if !self.is_set() => {
                self.path = Some(meta.path.to_token_stream());
                self.input = Some(token);
                self.value = Some(val);
            }
            Ok(_) => {}
            Err(e) => cx.syn_error(e),
        }
    }
}

impl<T> Attr for AttrVal<T> {
    fn attr(&self) -> Symbol {
        self.attr
    }

    fn name(&self) -> Symbol {
        self.name
    }

    fn is_set(&self) -> bool {
        self.value.is_some()
    }

    fn path_token(&self) -> Option<TokenStream> {
        self.path.clone()
    }
}

impl AttrBool {
    pub fn new(attr: Symbol, name: Symbol) -> Self {
        Self {
            attr,
            name,
            path: None,
        }
    }

    pub fn get(&self) -> bool {
        self.path.is_some()
    }

    pub fn try_from_meta(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>) {
        if self.is_set() {
            let msg = format!("duplicate {} attribute `{}`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);
        }

        if !self.is_set() {
            self.path = Some(meta.path.to_token_stream())
        }
    }
}

impl Attr for AttrBool {
    fn attr(&self) -> Symbol {
        self.attr
    }

    fn name(&self) -> Symbol {
        self.name
    }

    fn is_set(&self) -> bool {
        self.path.is_some()
    }

    fn path_token(&self) -> Option<TokenStream> {
        self.path.clone()
    }
}

impl<T> AttrVec<T> {
    pub fn new(attr: Symbol, name: Symbol) -> Self {
        Self {
            attr,
            name,
            inner: Vec::new(),
        }
    }

    pub fn get(&self) -> Vec<&T> {
        self.inner.iter().map(|x| &x.value).collect()
    }

    pub fn try_from_meta(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>)
    where
        T: Parse,
    {
        self.try_from_meta_map(cx, meta, |t| Result::<T, &str>::Ok(t))
    }

    pub fn try_from_meta_map<V, F, E>(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>, f: F)
    where
        V: Parse,
        F: Fn(V) -> Result<T, E>,
        E: core::fmt::Display,
    {
        match parse_meta(meta, f) {
            Ok((val, token)) => {
                let i = AttrVecInner {
                    path: meta.path.to_token_stream(),
                    input: token,
                    value: val,
                };
                self.inner.push(i);
            }
            Err(e) => cx.syn_error(e),
        }
    }

    pub fn check_dup_val(&self, cx: &Ctxt) {
        for (i, x) in self.inner.iter().enumerate() {
            for (_, y) in self.inner.iter().enumerate().take(i) {
                if x.input.to_string() == y.input.to_string() {
                    let msg = format!(
                        "duplicate {} attribute `{}` value `{}`",
                        self.attr, self.name, x.input
                    );
                    cx.error_spanned_by(&x.input, msg);
                }
            }
        }
    }
}

impl<T> Attr for AttrVec<T> {
    fn attr(&self) -> Symbol {
        self.attr
    }

    fn name(&self) -> Symbol {
        self.name
    }

    fn is_set(&self) -> bool {
        !self.inner.is_empty()
    }

    fn path_token(&self) -> Option<TokenStream> {
        self.inner.first().map(|x| x.path.clone())
    }
}

impl DeriveAttr {
    pub fn new(attr: Symbol, name: Symbol) -> Self {
        Self {
            attr,
            name,
            name_val: None,
            vis_val: None,
            enabled: None,
            path: None,
        }
    }

    pub fn name_val(&self) -> Option<&syn::Ident> {
        self.name_val.as_ref()
    }

    pub fn vis_val(&self) -> Option<&syn::Visibility> {
        self.vis_val.as_ref()
    }

    pub fn enabled_or(&self, def: bool) -> bool {
        self.enabled.unwrap_or(def)
    }

    /// Parse `name = <ident>`, `vis = "..."`, `enable`, `disable` from the
    /// nested meta of this attribute, e.g. `as_name(name = as_name, vis = "pub", disable)`.
    pub fn try_from_meta(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>) {
        use crate::symbol::*;

        if self.is_set() {
            let msg = format!("duplicate {} attribute `{}`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);

            // still consume the input so the outer
            // `parse_nested_meta` driver does not choke on leftover tokens
            let _ = meta.parse_nested_meta(|_| Ok(()));
            return;
        }

        self.path = Some(meta.path.to_token_stream());

        let res = meta.parse_nested_meta(|m| {
            if m.path == NAME {
                self.parse_name(cx, &m);
            } else if m.path == VIS {
                self.parse_vis(cx, &m);
            } else if m.path == ENABLE {
                self.set_enabled(cx, &m, true);
            } else if m.path == DISABLE {
                self.set_enabled(cx, &m, false);
            } else {
                let path = m.path.to_token_stream().to_string().replace(' ', "");
                let msg = format!(
                    "unknown {} `{}` sub-attribute `{}`",
                    self.attr, self.name, path
                );
                let err = m.error(msg);
                // Drain the rest of this sub-attribute so syn's `ParseBuffer`
                // drop handler doesn't surface a spurious "unexpected token"
                // error from the leftover `= value` or `(...)` tokens.
                let _ = m.input.parse::<proc_macro2::TokenStream>();
                return Err(err);
            }
            Ok(())
        });

        if let Err(err) = res {
            cx.syn_error(err);
        }
    }

    pub fn into_gen_option(self, def_opt: DefaultGenOption) -> Option<GenOption> {
        if !self.enabled_or(def_opt.enabled) {
            return None;
        }

        Some(GenOption {
            ident: self.name_val.unwrap_or(def_opt.ident),
            vis: self.vis_val.unwrap_or(def_opt.vis),
        })
    }

    fn parse_name(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>) {
        let is_dup = self.name_val.is_some();
        if is_dup {
            let msg = format!("duplicate {} `{}` value `name`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);
        }

        match parse_meta(meta, |i: syn::Ident| Result::<_, &str>::Ok(i)) {
            Ok((val, _token)) if !is_dup => {
                self.name_val = Some(val);
            }
            Ok(_) => {}
            Err(e) => cx.syn_error(e),
        }
    }

    fn parse_vis(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>) {
        let is_dup = self.vis_val.is_some();
        if is_dup {
            let msg = format!("duplicate {} `{}` value `vis`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);
        }

        match parse_meta(meta, |s: syn::LitStr| {
            syn::parse_str::<syn::Visibility>(&s.value())
        }) {
            Ok((val, _token)) if !is_dup => {
                self.vis_val = Some(val);
            }
            Ok(_) => {}
            Err(e) => cx.syn_error(e),
        }
    }

    fn set_enabled(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>, value: bool) {
        if let Some(prev_value) = self.enabled {
            let label = if value { "enable" } else { "disable" };
            let other = if value { "disable" } else { "enable" };
            let msg = if value == prev_value {
                format!("duplicate {} `{}` value `{}`", self.attr, self.name, label)
            } else {
                format!(
                    "{} `{}` value `{}` conflicts with `{}`",
                    self.attr, self.name, label, other
                )
            };
            cx.error_spanned_by(&meta.path, msg);
            return;
        }

        self.enabled = Some(value);
    }
}

impl Attr for DeriveAttr {
    fn attr(&self) -> Symbol {
        self.attr
    }

    fn name(&self) -> Symbol {
        self.name
    }

    fn is_set(&self) -> bool {
        self.path.is_some()
    }

    fn path_token(&self) -> Option<TokenStream> {
        self.path.clone()
    }
}

impl ImplAttr {
    pub fn new(attr: Symbol, name: Symbol) -> Self {
        Self {
            attr,
            name,
            enabled: None,
            path: None,
        }
    }

    pub fn enabled_or(&self, def: bool) -> bool {
        self.enabled.unwrap_or(def)
    }

    /// Parse `enable` / `disable` from the nested meta of this attribute,
    /// e.g. `impl_from_str(disable)`.
    pub fn try_from_meta(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>) {
        use crate::symbol::*;

        if self.is_set() {
            let msg = format!("duplicate {} attribute `{}`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);

            let _ = meta.parse_nested_meta(|_| Ok(()));
            return;
        }

        self.path = Some(meta.path.to_token_stream());

        let res = meta.parse_nested_meta(|m| {
            if m.path == ENABLE {
                self.set_enabled(cx, &m, true);
            } else if m.path == DISABLE {
                self.set_enabled(cx, &m, false);
            } else {
                let path = m.path.to_token_stream().to_string().replace(' ', "");
                let msg = format!(
                    "unknown {} `{}` sub-attribute `{}`",
                    self.attr, self.name, path
                );
                let err = m.error(msg);
                // Drain the rest of this sub-attribute so syn's `ParseBuffer`
                // drop handler doesn't surface a spurious "unexpected token"
                // error from the leftover `= value` or `(...)` tokens.
                let _ = m.input.parse::<proc_macro2::TokenStream>();
                return Err(err);
            }
            Ok(())
        });

        if let Err(err) = res {
            cx.syn_error(err);
        }
    }

    fn set_enabled(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>, value: bool) {
        if let Some(prev_value) = self.enabled {
            let label = if value { "enable" } else { "disable" };
            let other = if value { "disable" } else { "enable" };
            let msg = if value == prev_value {
                format!("duplicate {} `{}` value `{}`", self.attr, self.name, label)
            } else {
                format!(
                    "{} `{}` value `{}` conflicts with `{}`",
                    self.attr, self.name, label, other
                )
            };
            cx.error_spanned_by(&meta.path, msg);
            return;
        }

        self.enabled = Some(value);
    }
}

impl Attr for ImplAttr {
    fn attr(&self) -> Symbol {
        self.attr
    }

    fn name(&self) -> Symbol {
        self.name
    }

    fn is_set(&self) -> bool {
        self.path.is_some()
    }

    fn path_token(&self) -> Option<TokenStream> {
        self.path.clone()
    }
}

fn parse_meta<T, F, I, E>(meta: &ParseNestedMeta<'_>, f: F) -> syn::Result<(T, TokenStream)>
where
    F: Fn(I) -> Result<T, E>,
    I: Parse,
    E: core::fmt::Display,
{
    let value = meta.value()?;
    let tt: TokenTree = value.parse()?;
    let token: TokenStream = tt.into();
    let parsed: I = syn::parse2(token.clone())?;
    let val = f(parsed).map_err(|e| meta.error(e))?;
    Ok((val, token))
}
