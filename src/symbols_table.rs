use std::collections::HashMap;
use types::*;
use types::rlisperror::RlispErrorKind;
use lisp::Lisp;
use types::conversions::FromUnchecked;
use lisp::allocate::AllocObject;

pub trait SymbolLookup: AllocObject {
    fn push_namespace(&mut self, nmspc: *mut Namespace) {
        self.scope_mut().push(nmspc);
    }
    fn new_scope(&mut self, keys_and_vals: &[(*const Symbol, Object)]) {
        let mut table = HashMap::with_capacity(keys_and_vals.len());
        for (key, val) in keys_and_vals.iter().cloned() {
            let _insert_res = table.insert(key, val);
            debug_assert!(_insert_res.is_none());
        }
        let table = self.alloc(Namespace::from(table));
        let table = unsafe { <*mut Namespace as FromUnchecked<Object>>::from_unchecked(table) };
        self.scope_mut().push(table);
    }
    fn end_scope(&mut self) {
        self.scope_mut().pop();
        debug_assert!(!self.scope().is_empty());
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
                <&mut Symbol as FromUnchecked<Object>>::from_unchecked(obj)
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
            for table in self.scope() {
                let table = &**table;
                if table.contains_key(&sym) {
                    return *(table.get(&sym).unwrap());
                }
            }
            self.alloc(RlispError::unbound_symbol(Object::from(sym)))
        }
    }
    fn set_symbol(&mut self, sym: *const Symbol, val: Object) {
        for table in self.scope_mut() {
            let table = unsafe { &mut **table };
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
    unsafe fn type_from_symbol(&mut self, sym: *const Symbol) -> Option<RlispType> {
        let sym_name: &str = (*sym).as_ref();
        match sym_name {
            "cons" => Some(RlispType::Cons),
            "number" => Some(RlispType::Num),
            "symbol" => Some(RlispType::Sym),
            "string" => Some(RlispType::String),
            "function" => Some(RlispType::Function),
            "boolean" => Some(RlispType::Bool),
            "error" => Some(RlispType::Error),
            "integer" => Some(RlispType::Integer),
            "natnum" => Some(RlispType::NatNum),
            "namespace" => Some(RlispType::Namespace),
            _ => None,
        }
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
            RlispType::Namespace => "namespace",
        }))
    }
    fn error_name(&mut self, err: &RlispErrorKind) -> Object {
        Object::from(self.make_symbol(match *err {
            RlispErrorKind::WrongType { .. } => "wrong-type-error",
            RlispErrorKind::BadArgsCount { .. } => "wrong-arg-count-error",
            RlispErrorKind::ImproperList => "improper-list-error",
            RlispErrorKind::UnboundSymbol { .. } => "unbound-symbol-error",
            RlispErrorKind::RustError(_) => "internal-error",
            RlispErrorKind::Custom { kind, .. } => {
                return kind;
            }
        }))
    }
    fn scope(&self) -> &Scope;
    fn scope_mut(&mut self) -> &mut Scope;
    fn global_symbol_tab(&mut self) -> &mut Namespace {
        unsafe { &mut *(self.scope_mut()[0]) }
    }
    fn syms_in_memory(&mut self) -> &mut HashMap<String, *const Symbol>;
}

impl SymbolLookup for Lisp {
    fn scope(&self) -> &Scope {
        &self.symbols
    }
    fn scope_mut(&mut self) -> &mut Scope {
        &mut self.symbols
    }
    fn syms_in_memory(&mut self) -> &mut HashMap<String, *const Symbol> {
        &mut self.syms_in_memory
    }
}
