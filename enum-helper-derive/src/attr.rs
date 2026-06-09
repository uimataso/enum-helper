#![allow(dead_code)]

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{meta::ParseNestedMeta, parse::Parse};

use crate::{ctxt::Ctxt, symbol::Symbol};

pub trait Attr {
    fn attr(&self) -> Symbol;
    fn name(&self) -> Symbol;
    fn is_set(&self) -> bool;
    fn path_token(&self) -> Option<TokenStream>;
}

pub struct AttrVal<T> {
    attr: Symbol,
    name: Symbol,
    inner: Option<AttrValInner<T>>,
}

struct AttrValInner<T> {
    path: TokenStream,
    input: TokenStream,
    value: T,
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

pub struct AttrBool {
    attr: Symbol,
    name: Symbol,
    inner: Option<AttrBoolInner>,
}

struct AttrBoolInner {
    path: TokenStream,
}

impl<T> AttrVal<T> {
    pub fn new(attr: Symbol, name: Symbol) -> Self {
        Self {
            attr,
            name,
            inner: None,
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.inner.as_ref().map(|x| &x.value)
    }

    pub fn try_from_meta<V, F, E>(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>, f: F)
    where
        V: Parse,
        F: Fn(V) -> Result<T, E>,
        E: core::fmt::Display,
    {
        // we need to parse the value every time so the error msg is correct
        // so even if the value is set, we still need to parse the value
        if self.inner.is_some() {
            let msg = format!("duplicate {} attribute `{}`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);
        }

        match self.parse_meta(meta, f) {
            Ok(t) => {
                if self.inner.is_none() {
                    self.inner = Some(t);
                }
            }
            Err(e) => cx.syn_error(e),
        }
    }

    fn parse_meta<V, F, E>(
        &mut self,
        meta: &ParseNestedMeta<'_>,
        f: F,
    ) -> syn::Result<AttrValInner<T>>
    where
        V: Parse,
        F: Fn(V) -> Result<T, E>,
        E: core::fmt::Display,
    {
        let value = meta.value()?;
        let token: TokenStream = value.parse()?;
        let parsed: V = syn::parse2(token.clone())?;
        let res = f(parsed).map_err(|e| meta.error(e))?;
        Ok(AttrValInner {
            path: meta.path.to_token_stream(),
            input: token,
            value: res,
        })
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
        self.inner.is_some()
    }

    fn path_token(&self) -> Option<TokenStream> {
        self.inner.as_ref().map(|x| x.path.clone())
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

    pub fn get_owned(self) -> Vec<T> {
        self.inner.into_iter().map(|x| x.value).collect()
    }

    pub fn try_from_meta<V, F, E>(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>, f: F)
    where
        V: Parse,
        F: Fn(V) -> Result<T, E>,
        E: core::fmt::Display,
    {
        match self.parse_meta(meta, f) {
            Ok(t) => self.inner.push(t),
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

    fn parse_meta<V, F, E>(
        &mut self,
        meta: &ParseNestedMeta<'_>,
        f: F,
    ) -> syn::Result<AttrVecInner<T>>
    where
        V: Parse,
        F: Fn(V) -> Result<T, E>,
        E: core::fmt::Display,
    {
        let value = meta.value()?;
        let token: TokenStream = value.parse()?;
        let parsed: V = syn::parse2(token.clone())?;
        let res = f(parsed).map_err(|e| meta.error(e))?;
        Ok(AttrVecInner {
            path: meta.path.to_token_stream(),
            input: token,
            value: res,
        })
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

impl AttrBool {
    pub fn new(attr: Symbol, name: Symbol) -> Self {
        Self {
            attr,
            name,
            inner: None,
        }
    }

    pub fn get(&self) -> bool {
        self.inner.is_some()
    }

    pub fn try_from_meta(&mut self, cx: &Ctxt, meta: &ParseNestedMeta<'_>) {
        if self.inner.is_some() {
            let msg = format!("duplicate {} attribute `{}`", self.attr, self.name);
            cx.error_spanned_by(&meta.path, msg);
        }

        match self.parse_meta(meta) {
            Ok(t) => {
                if self.inner.is_none() {
                    self.inner = Some(t)
                }
            }
            Err(e) => cx.syn_error(e),
        }
    }

    fn parse_meta(&mut self, meta: &ParseNestedMeta<'_>) -> syn::Result<AttrBoolInner> {
        Ok(AttrBoolInner {
            path: meta.path.to_token_stream(),
        })
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
        self.inner.is_some()
    }

    fn path_token(&self) -> Option<TokenStream> {
        self.inner.as_ref().map(|x| x.path.clone())
    }
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
