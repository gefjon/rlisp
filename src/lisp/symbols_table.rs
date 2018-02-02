use std::collections::HashMap;
use types::*;
use types::rlisperror::RlispErrorKind;
use lisp::Lisp;
use lisp::allocate::AllocObject;

pub trait Symbols: AllocObject {
    fn new_scope(&mut self, keys_and_vals: &[(*const Symbol, Object)]) {
        let mut table = HashMap::with_capacity(keys_and_vals.len());
        for (key, val) in keys_and_vals.iter().cloned() {
            let _insert_res = table.insert(key, val);
            debug_assert!(_insert_res.is_none());
        }
        self.symbol_tab_stack().push(table);
    }
    fn end_scope(&mut self) {
        self.symbol_tab_stack().pop();
        debug_assert!(!self.symbol_tab_stack().is_empty());
    }
    fn make_symbol<T>(&mut self, sym: T) -> *const Symbol
    where
        String: ::std::convert::From<T>,
        T: ::std::convert::AsRef<str>,
    {
        let sym = String::from(sym);
        if self.syms_in_memory().contains_key(&sym) {
            *(self.syms_in_memory().get(&sym).unwrap())
        } else {
            let new_symbol = unsafe {
                let obj = self.alloc_sym(sym.as_ref());
                <&mut Symbol as conversions::FromUnchecked<Object>>::from_unchecked(obj)
            };
            let _insert_res = self.syms_in_memory().insert(sym.clone(), new_symbol);
            debug_assert!(_insert_res.is_none());
            new_symbol
        }
    }
    unsafe fn get_symbol(&mut self, sym: *const Symbol) -> Object {
        let sym_name: &[u8] = (&*sym).as_ref();
        if sym_name == b"nil" {
            Object::nil()
        } else if sym_name == b"t" {
            Object::t()
        } else if sym_name[0] == b':' {
            Object::from(sym)
        } else {
            for table in self.symbol_tab_stack() {
                if table.contains_key(&sym) {
                    return *(table.get(&sym).unwrap());
                }
            }
            self.alloc(RlispError::unbound_symbol(Object::from(sym)))
        }
    }
    fn set_symbol(&mut self, sym: *const Symbol, val: Object) {
        for table in self.symbol_tab_stack().iter_mut() {
            if table.contains_key(&sym) {
                if let Some(sym_val) = table.get_mut(&sym) {
                    *sym_val = val;
                } else {
                    unreachable!();
                }
                return;
            }
        }
        let _insert_res = self.global_symbol_tab().insert(sym, val);
        debug_assert!(_insert_res.is_none());
    }

    fn type_name(&mut self, typ: RlispType) -> Object {
        Object::from(self.make_symbol(match typ {
            RlispType::Cons => "cons",
            RlispType::Num => "number",
            RlispType::Sym => "symbol",
            RlispType::String => "string",
            RlispType::Function => "function",
            RlispType::Bool => "boolean",
            RlispType::Error => "error",
            RlispType::Integer => "integer",
            RlispType::NatNum => "natnum",
        }))
    }
    fn error_name(&mut self, err: &RlispErrorKind) -> Object {
        Object::from(self.make_symbol(match *err {
            RlispErrorKind::WrongType { .. } => "wrong-type-error",
            RlispErrorKind::BadArgsCount { .. } => "wrong-arg-count-error",
            RlispErrorKind::ImproperList => "improper-list-error",
            RlispErrorKind::UnboundSymbol { .. } => "unbound-symbol-error",
            RlispErrorKind::RustError(_) => "internal-error",
        }))
    }
    fn symbol_tab_stack(&mut self) -> &mut Vec<HashMap<*const Symbol, Object>>;
    fn global_symbol_tab(&mut self) -> &mut HashMap<*const Symbol, Object> {
        &mut self.symbol_tab_stack()[0]
    }
    fn syms_in_memory(&mut self) -> &mut HashMap<String, *const Symbol>;
}

impl Symbols for Lisp {
    fn symbol_tab_stack(&mut self) -> &mut Vec<HashMap<*const Symbol, Object>> {
        &mut self.symbols
    }
    fn syms_in_memory(&mut self) -> &mut HashMap<String, *const Symbol> {
        &mut self.syms_in_memory
    }
}
