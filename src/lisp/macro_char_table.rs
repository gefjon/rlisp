use lisp;
use types::*;
use symbols_table::SymbolLookup;

pub trait MacroChars: SymbolLookup {
    fn check_macro_char(&mut self, byte: u8) -> Option<Object>;
}

impl MacroChars for lisp::Lisp {
    fn check_macro_char(&mut self, byte: u8) -> Option<Object> {
        let symbol = {
            if let Some(sym_str) = self.macro_chars.get(&byte) {
                *sym_str
            } else {
                return None;
            }
        };
        Some(Object::from(self.make_symbol(symbol)))
    }
}
