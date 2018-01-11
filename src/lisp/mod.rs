use std::collections::HashMap;
use std::default::Default;

use types::*;

mod macro_char_table;
pub use self::macro_char_table::MacroChars;

mod symbols_table;
pub use self::symbols_table::Symbols;

const INITIAL_MACRO_CHARS: &[(u8, &str)] =
    &[(b'\'', "quote"), (b'`', "backquote"), (b',', "comma")];

pub struct Lisp {
    symbols: symbols_table::SymbolsTab,
    macro_chars: HashMap<u8, &'static str>,
    conses: Vec<ConsCell>,
    strings: Vec<String>,
}

pub trait Store<T> {
    fn store(&mut self, to_store: T) -> Object;
}

impl Store<ConsCell> for Lisp {
    fn store(&mut self, to_store: ConsCell) -> Object {
        self.conses.push(to_store);
        if let Some(stored) = self.conses.last() {
            Object::from(stored as *const ConsCell)
        } else {
            unreachable!()
        }
    }
}

impl Store<String> for Lisp {
    fn store(&mut self, to_store: String) -> Object {
        self.strings.push(to_store);
        if let Some(stored) = self.strings.last() {
            Object::from(stored as *const String)
        } else {
            unreachable!()
        }
    }
}

impl Default for Lisp {
    fn default() -> Self {
        Self {
            symbols: symbols_table::SymbolsTab::default(),
            macro_chars: INITIAL_MACRO_CHARS.iter().cloned().collect(),
            conses: Vec::new(),
            strings: Vec::new(),
        }
    }
}
