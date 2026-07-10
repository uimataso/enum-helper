use core::fmt;

use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const ALIAS_ALL: Symbol = Symbol("alias_all");
pub const ALIAS: Symbol = Symbol("alias");
pub const ALL_ALIASES: Symbol = Symbol("all_aliases");
pub const ALL_NAMES: Symbol = Symbol("all_names");
pub const ALL: Symbol = Symbol("all");
pub const AS_ALIASES: Symbol = Symbol("as_aliases");
pub const AS_NAME: Symbol = Symbol("as_name");
pub const ATTR: Symbol = Symbol("attr");
pub const COUNT: Symbol = Symbol("count");
pub const DEFAULT: Symbol = Symbol("default");
pub const DISABLE: Symbol = Symbol("disable");
pub const ENABLE: Symbol = Symbol("enable");
pub const ENUM_ALL: Symbol = Symbol("enum_all");
pub const ENUM_KIND: Symbol = Symbol("enum_kind");
pub const ENUM_STR: Symbol = Symbol("enum_str");
pub const ERROR_MSG: Symbol = Symbol("error_msg");
pub const ERROR: Symbol = Symbol("error");
pub const IMPL_AS_REF_STR: Symbol = Symbol("impl_as_ref_str");
pub const IMPL_DISPLAY: Symbol = Symbol("impl_display");
pub const IMPL_FROM_STR: Symbol = Symbol("impl_from_str");
pub const IMPL_INTO_STATIC_STR: Symbol = Symbol("impl_into_static_str");
pub const IMPL_TRY_FROM_STR: Symbol = Symbol("impl_try_from_str");
pub const NAME: Symbol = Symbol("name");
pub const NO_DEFAULT_DERIVE: Symbol = Symbol("no_default_derive");
pub const NO_PARSING: Symbol = Symbol("no_parsing");
pub const NO_RENDERING: Symbol = Symbol("no_rendering");
pub const RENAME_ALL: Symbol = Symbol("rename_all");
pub const RENAME: Symbol = Symbol("rename");
pub const SKIP: Symbol = Symbol("skip");
pub const VIS: Symbol = Symbol("vis");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl PartialEq<Symbol> for &Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl PartialEq<Symbol> for &Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
