use std::collections::HashMap;
use types::*;

pub struct SymbolsTab {
    map: HashMap<String, Symbol>,
}

impl SymbolsTab {
    pub fn new() -> Self {
        SymbolsTab {
            map: HashMap::new(),
        }
    }
    pub fn intern_str(&mut self, sym: &str) -> &Symbol {
        self.intern(String::from(sym))
    }
    pub fn intern(&mut self, sym: String) -> &Symbol {
        if !self.map.contains_key(&sym) {
            let new_symbol = Symbol::from_string(sym.clone());
            let _ = self.map.insert(sym.clone(), new_symbol);
        }
        if let Some(symbol) = self.map.get(&sym) {
            symbol
        } else {
            unreachable!()
        }
    }
}
