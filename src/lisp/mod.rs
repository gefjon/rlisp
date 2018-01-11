use std::collections::HashMap;

mod macro_char_table;
pub use self::macro_char_table::MacroChars;

mod symbols_table;
pub use self::symbols_table::Symbols;

const INITIAL_MACRO_CHARS: &[(u8, &str)] =
    &[(b'\'', "quote"), (b'`', "backquote"), (b',', "comma")];

pub struct Lisp {
    symbols: symbols_table::SymbolsTab,
    macro_chars: HashMap<u8, &'static str>,
}

impl Lisp {
    pub fn new() -> Self {
        Self {
            symbols: symbols_table::SymbolsTab::new(),
            macro_chars: INITIAL_MACRO_CHARS.iter().cloned().collect(),
        }
    }
}
