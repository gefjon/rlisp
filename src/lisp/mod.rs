use std::collections::HashMap;
use std::default::Default;
use types::*;
use types::into_object::*;
use types::conversions::*;
use builtins;
use std::convert;
use symbols_table::SymbolLookup;

mod macro_char_table;
pub use self::macro_char_table::MacroChars;

pub mod stack_storage {
    use types::*;
    use result::*;
    use lisp;
    use lisp::allocate::AllocObject;
    pub trait Stack {
        fn push(&mut self, obj: Object);
        fn pop(&mut self) -> Object;
        fn stack_vec(&self) -> &Vec<Object>;
        fn clean_stack(&mut self);
    }
    impl Stack for lisp::Lisp {
        fn push(&mut self, obj: Object) {
            self.stack.push(obj);
        }
        fn pop(&mut self) -> Object {
            if let Some(obj) = self.stack.pop() {
                obj
            } else {
                let e: Error = ErrorKind::StackUnderflow.into();
                let e: RlispError = e.into();
                self.alloc(e)
            }
        }
        fn stack_vec(&self) -> &Vec<Object> {
            &self.stack
        }
        fn clean_stack(&mut self) {
            self.stack = Vec::new();
        }
    }
}

pub mod allocate;

const INITIAL_MACRO_CHARS: &[(u8, &str)] = &[(b'\'', "quote")];

pub struct Lisp {
    pub symbols: Scope,
    pub syms_in_memory: HashMap<String, *const Symbol>,
    macro_chars: HashMap<u8, &'static str>,
    pub stack: Vec<Object>,
    pub current_gc_mark: ::gc::GcMark,
    pub alloced_objects: Vec<Object>,
    pub gc_threshold: usize,
}

impl Lisp {
    fn source_builtins(&mut self, mut builtin_funcs: builtins::RlispBuiltins) {
        use lisp::allocate::AllocObject;
        use list::ListOps;
        for (name, mut arglist, fun) in builtin_funcs.drain(..) {
            let name = self.make_symbol(name);
            let arglist = {
                let mut arg_syms = Vec::new();
                for arg in arglist.drain(..) {
                    arg_syms.push(Object::from(self.make_symbol(arg)));
                }
                self.list_from_vec(arg_syms)
            };
            let fun = self.alloc(
                RlispFunc::from_builtin(fun)
                    .with_name(Object::from(name))
                    .with_arglist(arglist),
            );
            self.set_symbol(name, fun);
        }
    }
    fn source_special_forms(&mut self, mut special_forms: builtins::RlispSpecialForms) {
        use lisp::allocate::AllocObject;
        use list::ListOps;
        for (name, mut arglist, fun) in special_forms.drain(..) {
            let name = self.make_symbol(name);
            let arglist = {
                let mut arg_syms = Vec::new();
                for arg in arglist.drain(..) {
                    arg_syms.push(Object::from(self.make_symbol(arg)));
                }
                self.list_from_vec(arg_syms)
            };
            let fun = self.alloc(
                RlispFunc::from_special_form(fun)
                    .with_name(Object::from(name))
                    .with_arglist(arglist),
            );
            self.set_symbol(name, fun);
        }
    }
    fn source_builtin_vars(&mut self, mut builtin_vars: builtins::RlispBuiltinVars) {
        for (name, val) in builtin_vars.drain(..) {
            let name = self.make_symbol(name);
            let val = self.convert_into_object(val);
            self.set_symbol(name, val)
        }
    }
}

impl Default for Lisp {
    fn default() -> Self {
        use lisp::allocate::AllocObject;
        let mut me = Self {
            symbols: vec![],
            syms_in_memory: HashMap::new(),
            macro_chars: INITIAL_MACRO_CHARS.iter().cloned().collect(),
            current_gc_mark: 1,
            stack: Vec::new(),
            alloced_objects: Vec::new(),
            gc_threshold: 16,
        };
        let global_namespace = me.alloc(Namespace::default());
        let global_namespace = unsafe { global_namespace.into_unchecked() };
        me.push_namespace(global_namespace);
        me.source_builtin_vars(builtins::builtin_vars());
        me.source_special_forms(builtins::make_special_forms());
        me.source_builtins(builtins::make_builtins());
        me.source_builtins(::math::math_builtins::make_builtins());
        me
    }
}

impl convert::AsMut<Lisp> for Lisp {
    fn as_mut(&mut self) -> &mut Lisp {
        self
    }
}
