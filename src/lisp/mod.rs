use std::collections::HashMap;
use std::default::Default;
use types::*;
use builtins;

mod macro_char_table;
pub use self::macro_char_table::MacroChars;

mod symbols_table;
pub use self::symbols_table::Symbols;

pub mod stack_storage {
    use types::Object;
    use result::*;
    use lisp;
    pub trait Stack {
        fn push(&mut self, obj: Object);
        fn pop(&mut self) -> Result<Object>;
        fn stack_vec(&self) -> &Vec<Object>;
    }
    impl Stack for lisp::Lisp {
        fn push(&mut self, obj: Object) {
            self.stack.push(obj);
        }
        fn pop(&mut self) -> Result<Object> {
            if let Some(obj) = self.stack.pop() {
                Ok(obj)
            } else {
                Err(ErrorKind::StackUnderflow.into())
            }
        }
        fn stack_vec(&self) -> &Vec<Object> {
            &self.stack
        }
    }
}

pub mod allocate {
    use types::*;
    use std::boxed::Box;
    use lisp;
    pub trait AllocObject {
        fn alloc<T>(&mut self, to_alloc: T) -> Object
        where
            Object: ::std::convert::From<*mut T>,
        {
            let boxed = Box::new(to_alloc);
            let obj = Object::from(Box::into_raw(boxed));
            self.objects_mut().push(obj);
            obj
        }
        fn objects(&self) -> &Vec<Object>;
        fn objects_mut(&mut self) -> &mut Vec<Object>;
    }
    impl AllocObject for lisp::Lisp {
        fn objects(&self) -> &Vec<Object> {
            &self.alloced_objects
        }
        fn objects_mut(&mut self) -> &mut Vec<Object> {
            &mut self.alloced_objects
        }
    }
}

const INITIAL_MACRO_CHARS: &[(u8, &str)] =
    &[(b'\'', "quote"), (b'`', "backquote"), (b',', "comma")];

pub struct Lisp {
    pub symbols: HashMap<String, *const Symbol>,
    macro_chars: HashMap<u8, &'static str>,
    pub stack: Vec<Object>,
    pub current_gc_mark: ::gc::GcMark,
    pub alloced_objects: Vec<Object>,
}

impl Lisp {
    fn source_builtins(&mut self, mut builtin_funcs: Vec<builtins::RlispBuiltinTuple>) {
        use lisp::allocate::AllocObject;
        for (name, fun) in builtin_funcs.drain(..) {
            let fun = self.alloc(RlispFunc::from_builtin(fun));
            if let Some(sym) = self.intern(name).into_symbol_mut() {
                sym.set(fun);
            } else {
                unreachable!()
            }
        }
    }
}

impl Default for Lisp {
    fn default() -> Self {
        let mut me = Self {
            symbols: HashMap::new(),
            macro_chars: INITIAL_MACRO_CHARS.iter().cloned().collect(),
            current_gc_mark: 1,
            stack: Vec::new(),
            alloced_objects: Vec::new(),
        };
        me.source_builtins(builtins::make_builtins());
        me
    }
}
