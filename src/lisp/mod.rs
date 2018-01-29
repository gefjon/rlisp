use std::collections::HashMap;
use std::default::Default;
use types::*;
use types::conversions::*;
use builtins;
use std::convert;

mod macro_char_table;
pub use self::macro_char_table::MacroChars;

pub mod symbols_table;
pub use self::symbols_table::Symbols;

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

const INITIAL_MACRO_CHARS: &[(u8, &str)] = &[(b'\'', "quote")];

pub struct Lisp {
    pub symbols: HashMap<String, *const Symbol>,
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
            let name = self.intern(name);
            let arglist = {
                let mut arg_syms = Vec::new();
                for arg in arglist.drain(..) {
                    arg_syms.push(self.intern(arg));
                }
                self.list_from_vec(arg_syms)
            };
            let fun = self.alloc(
                RlispFunc::from_builtin(fun)
                    .with_name(name)
                    .with_arglist(arglist),
            );
            unsafe {
                <Object as IntoUnchecked<&mut Symbol>>::into_unchecked(name).set(fun);
            }
        }
    }
    fn source_special_forms(&mut self, mut special_forms: builtins::RlispSpecialForms) {
        use lisp::allocate::AllocObject;
        use list::ListOps;
        for (name, mut arglist, fun) in special_forms.drain(..) {
            let name = self.intern(name);
            let arglist = {
                let mut arg_syms = Vec::new();
                for arg in arglist.drain(..) {
                    arg_syms.push(self.intern(arg));
                }
                self.list_from_vec(arg_syms)
            };
            let fun = self.alloc(
                RlispFunc::from_special_form(fun)
                    .with_name(name)
                    .with_arglist(arglist),
            );
            unsafe {
                <Object as IntoUnchecked<&mut Symbol>>::into_unchecked(name).set(fun);
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
            gc_threshold: 16,
        };
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
