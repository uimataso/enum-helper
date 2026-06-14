use core::fmt;

use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const ALIAS_ALL: Symbol = Symbol("alias_all");
pub const ALIAS: Symbol = Symbol("alias");
pub const ATTR: Symbol = Symbol("attr");
pub const DEFAULT: Symbol = Symbol("default");
pub const ENUM_ALL: Symbol = Symbol("enum_all");
pub const ENUM_KIND: Symbol = Symbol("enum_kind");
pub const ENUM_STR: Symbol = Symbol("enum_str");
pub const ERROR_MSG: Symbol = Symbol("error_msg");
pub const ERROR_NAME: Symbol = Symbol("error_name");
pub const NAME: Symbol = Symbol("name");
pub const NO_DEFAULT_DERIVE: Symbol = Symbol("no_default_derive");
pub const NO_ERROR_STRUCT: Symbol = Symbol("no_error_struct");
pub const NO_PARSING: Symbol = Symbol("no_parsing");
pub const NO_RENDERING: Symbol = Symbol("no_rendering");
pub const RENAME_ALL: Symbol = Symbol("rename_all");
pub const RENAME: Symbol = Symbol("rename");
pub const SKIP: Symbol = Symbol("skip");

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
