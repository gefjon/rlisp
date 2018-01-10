mod symbols_table;
pub use self::symbols_table::Symbols;

pub struct Lisp {
    pub symbols: symbols_table::SymbolsTab,
}

impl Lisp {
    pub fn new() -> Self {
        Self {
            symbols: symbols_table::SymbolsTab::new(),
        }
    }
}
