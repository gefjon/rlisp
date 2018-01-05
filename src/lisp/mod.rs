mod symbols_table;

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
