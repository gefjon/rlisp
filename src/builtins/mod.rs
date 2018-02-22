use symbols_table::SymbolLookup;
use result::*;
use types::*;
use types::into_object::*;
use lisp;
use std::boxed::Box;
use lisp::allocate::AllocObject;
use lisp::stack_storage::Stack;
use types::conversions::*;

// The macros `special_forms` and `builtin_functions` are the main
// part of this module. See `make_builtins()` and `make_special_forms`
// for their use.  Each function consists of a string name, a list of
// identifiers which will have args bound to them, and a block which
// returns an Object. The block can use the `?` operator, but should
// do so sparingly

pub type RlispBuiltinFunc = FnMut(&mut lisp::Lisp, i32) -> Object;
pub type RlispSpecialForm = FnMut(&mut lisp::Lisp, i32) -> Object;
pub type Name = &'static [u8];
pub type Arglist = Vec<Name>;
pub type RlispBuiltinTuple = (Name, Arglist, Box<RlispBuiltinFunc>);
pub type RlispSpecialForms = Vec<(Name, Arglist, Box<RlispSpecialForm>)>;
pub type RlispBuiltins = Vec<RlispBuiltinTuple>;
pub type RlispBuiltinVars = Vec<(Name, IntoObject)>;

/// returns the `RlispSpecialForms` used by Rlisp. This is basic
/// language functionality.
pub fn make_special_forms() -> RlispSpecialForms {
    use evaluator::Evaluator;
    special_forms!{
        l = lisp;
        a = args;
        "cond" (&rest clauses) -> {
            for clause in &a {
                let &ConsCell { car, cdr, .. } = into_type_or_error!(l : *clause => &ConsCell);
                if bool::from(l.evaluate(car)) {
                    let &ConsCell { car: cdrcar, .. } =  into_type_or_error!(l : cdr => &ConsCell);
                    return l.evaluate(cdrcar);
                }
            }
            false.into()
        },
        "let" (bindings &rest body) -> {
            let bindings = a[0];
            let mut scope = Vec::new();

            #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
            for binding_pair in into_type_or_error!(l : bindings => &ConsCell).into_iter() {
                let &ConsCell { car: symbol, cdr, .. } =
                    into_type_or_error!(l : binding_pair => &ConsCell);
                let &ConsCell { car: value, .. } =
                    into_type_or_error!(l : cdr => &ConsCell);
                scope.push((
                    into_type_or_error!(l : symbol => *const Symbol),
                    {
                        let r = l.evaluate(value);
                        bubble!(r);
                        r
                    }
                ));
            }
            l.new_scope(&scope);
            let mut res = Object::nil();
            for body_clause in &a[1..] {
                res = l.evaluate(*body_clause);
                bubble!(res);
            }
            l.end_scope();
            res
        },
        "setf" (place value &rest places values) -> {
            if ::math::oddp(a.len() as _) {
                let e: Error = ErrorKind::WantedEvenArgCt.into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let mut res = Object::nil();
                for i in (0..a.len()).step_by(2) {
                    let place = bubble!(l.evaluate(a[i]));
                    let mut place = into_type_or_error!(l : place => Place);
                    res = bubble!(l.evaluate(a[i + 1]));
                    *place = res;
                }
                res
            }
        },
        "nref" (namespace symbol) -> {
            let namespace = bubble!(l.evaluate(a[0]));
            let namespace = unsafe {
                &mut *(into_type_or_error!(l : namespace => *mut Namespace))
            };
            let sym = into_type_or_error!(l : a[1] => *const Symbol);
            Object::from(namespace.sym_ref(sym))
        },
        "setq" (symbol value &rest symbols values) -> {
            if ::math::oddp(a.len() as _) {
                let e: Error = ErrorKind::WantedEvenArgCt.into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let mut res = Object::nil();
                for i in (0..a.len()).step_by(2) {
                    let sym = into_type_or_error!(l : a[i] => *const Symbol);
                    let value = a[i + 1];
                    res = l.evaluate(value);
                    bubble!(res);
                    l.set_symbol(sym, res);
                }
                res
            }
        },
        "quote" (x) -> {
            if a.len() != 1 {
                let e: Error =
                    ErrorKind::WrongArgsCount(
                        a.len() as _,
                        1, Some(1)
                    ).into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                a[0]
            }
        },
        "if" (predicate ifclause &rest elseclauses) -> {
            let predicate = a[0];
            let if_clause = a[1];
            let else_clauses = &a[2..];
            if bool::from({
                let r = l.evaluate(predicate);
                bubble!(r);
                r
            }) {
                l.evaluate(if_clause)
            } else {
                let mut res = Object::nil();
                for clause in else_clauses {
                    res = l.evaluate(*clause);
                    bubble!(res);
                }
                res
            }
        },
        "defun" (name arglist &rest body) -> {
            let name = a[0];
            let name_sym = into_type_or_error!(l : name => *const Symbol);
            let arglist = a[1];
            let body = &a[2..];
            let scope = l.symbols.clone();
            let fun = l.alloc(
                RlispFunc::from_body(body.into())
                    .with_name(name)
                    .with_arglist(arglist)
                    .with_scope(scope)
            );
            l.set_symbol(name_sym, fun);
            fun
        },
        "defvar" (name value) -> {
            let name = into_type_or_error!(l : a[0] => *const Symbol);
            let val = a[1];
            let val = l.evaluate(val);
            bubble!(val);
            l.set_symbol(name, val);
            val
        },
        "catch-error" (statement &rest handlers) -> {
            let statement = a[0];

            let handlers = &a[1..];

            let res = l.evaluate(statement);

            if let Some(e) = <Object as MaybeInto<&RlispError>>::maybe_into(res) {
                let e = &e.error;
                let e = l.error_name(e);
                for handler in handlers {
                    let &ConsCell { car, cdr, .. } =
                        into_type_or_error!(l : *handler => &ConsCell);
                    if (car == e) || (car == Object::t()) {
                        let &ConsCell { car, .. } =
                            into_type_or_error!(l : cdr => &ConsCell);
                        return l.evaluate(car);
                    }
                }
            }
            res
        },
        "lambda" (args &rest body) -> {
            let arglist = a[0];
            let body = &a[1..];
            if (!arglist.nilp()) && (!<&ConsCell as FromObject>::is_type(arglist)) {
                let e = RlispError::wrong_type(l.type_name(RlispType::Cons),
                                               l.type_name(arglist.what_type()));
                l.alloc(e)
            } else {
                let scope = l.symbols.clone();
                l.alloc(
                    RlispFunc::from_body(body.into())
                        .with_arglist(arglist)
                        .with_scope(scope)
                )
            }
        },
        "check-type" (&rest objects typenames) -> {
            if ::math::oddp(a.len() as _) {
                let e: Error = ErrorKind::WantedEvenArgCt.into();
                let e: RlispError = e.into();
                l.alloc(e)
            } else {
                let mut res = Object::nil();
                for i in (0..a.len()).step_by(2) {
                    let obj = a[i];
                    let type_obj = a[i + 1];
                    let type_name = into_type_or_error!(l : type_obj => *const Symbol);
                    res = l.evaluate(obj);
                    bubble!(res);
                    if let Some(typ) = unsafe { l.type_from_symbol(type_name) } {
                        if !typ.check_type(res) {
                            let e = RlispError::wrong_type(l.type_name(typ),
                                                           l.type_name(res.what_type()));
                            return l.alloc(e);
                        }
                    } else {
                        let e = RlispError::not_a_type(type_obj);
                        return l.alloc(e);
                    }
                }
                res
            }
        },
        "get" (namespace symbol) -> {
            let namespace = a[0];
            let symbol = a[1];
            bubble!(symbol);
            let namespace = l.evaluate(namespace);
            let namespace = into_type_or_error!(l : namespace => &'static Namespace);
            let symbol = into_type_or_error!(l : symbol => *const Symbol);
            if let Some(obj) = namespace.get(&symbol) {
                *obj
            } else {
                l.alloc(RlispError::unbound_symbol(Object::from(symbol)))
            }
        },
        "set" (namespace symbol value) -> {
            let namespace = a[0];
            let symbol = a[1];
            bubble!(symbol);
            let symbol = into_type_or_error!(l : symbol => *const Symbol);
            let value = a[1];
            let namespace = l.evaluate(namespace);
            bubble!(namespace);
            let value = l.evaluate(value);
            bubble!(value);
            let namespace = into_type_or_error!(l : namespace => &'static mut Namespace);
            if let Some(obj) = namespace.insert(symbol, value) {
                obj
            } else {
                Object::nil()
            }
        },
        "make-namespace" (&optional name) -> {
            let name = if a.len() == 1 {
                Some(a[0])
            } else {
                None
            };
            let namespace = Namespace::default()
                .with_maybe_name(name);
            let namespace = l.alloc(namespace);
            if let Some(name) = name {
                if let Some(sym) = <*const Symbol>::maybe_from(name) {
                    l.set_symbol(sym, namespace);
                }
            }
            namespace
        },
    }
}

/// Returns the `RlispBuiltinFunctions` used by Rlisp which don't fit
/// better in `math::make_builtins`. Many of these functions relate to
/// type-checking, debugging, and basically anything that doesn't
/// operate exclusively on numbers.
pub fn make_builtins() -> RlispBuiltins {
    builtin_functions!{
        l = lisp;
        "consp" (c) -> { <&ConsCell>::is_type_or_place(c).into() },
        "numberp" (n) -> { RlispNum::is_type_or_place(n).into() },
        "integerp" (n) -> { i32::is_type_or_place(n).into() },
        "floatp" (n) -> { f64::is_type_or_place(n).into() },
        "symbolp" (s) -> { <&Symbol>::is_type_or_place(s).into() },
        "stringp" (s) -> { <&RlispString>::is_type_or_place(s).into() },
        "functionp" (f) -> { <&RlispFunc>::is_type_or_place(f).into() },
        "boolp" (b) -> { bool::is_type_or_place(b).into() },
        "namespacep" (n) -> { <&Namespace>::is_type_or_place(n).into() },
        "placep" (p) -> { Place::is_type(p).into() },
        "cons" (car cdr) -> { l.alloc(ConsCell::new(car, cdr)) },
        "list" (&rest items) -> { items },
        "debug" (obj) -> { println!("{:?}", obj); obj },
        "print" (&rest objects) -> {
            if let Some(cons) = <&ConsCell as MaybeFrom<_>>::maybe_from(objects) {
                let mut count: i32 = 0;
                for obj in cons {
                    print!("{}", obj);
                    count += 1;
                }
                println!();
                count.into()
            } else {
                false.into()
            }
        },
        "eq" (first &rest objects) -> {
            if let Some(cons) = <&ConsCell as MaybeFrom<_>>::maybe_from(objects) {
                #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                for el in cons.into_iter() {
                    if !(first == el) {
                        return false.into();
                    }
                }
            } else {
                debug_assert!(objects.nilp())
            }
            true.into()
        },
        "wrong-type-error" (wanted found) -> {
            l.alloc(RlispError::wrong_type(wanted, found))
        },
        "wrong-arg-count-error" (found min &optional max) -> {
            let _ = into_type_or_error!(l : found => i32);
            let _ = into_type_or_error!(l : found => i32);
            if bool::from(max) {
                let _ = into_type_or_error!(l : found => i32);
            }
            l.alloc(RlispError::bad_args_count(found, min, max))
        },
        "improper-list-error" () -> {
            l.alloc(RlispError::improper_list())
        },
        "unbound-symbol-error" (sym) -> {
            let _ = into_type_or_error!(l : sym => &'static Symbol);
            l.alloc(RlispError::unbound_symbol(sym))
        },
        "type-designator-error" (designator) -> {
            l.alloc(RlispError::not_a_type(designator))
        },
        "undefined-symbol-error" (sym) -> {
            let _ = into_type_or_error!(l : sym => *const Symbol);
            l.alloc(RlispError::undefined_symbol(sym))
        },
        "index-out-of-bounds-error" (idx reciever) -> {
            l.alloc(RlispError::index_out_of_bounds(idx, reciever))
        },
        "error" (kind &rest info) -> {
            l.alloc(RlispError::custom(kind, info))
        },
        "global-namespace" () -> {
            Object::from(l.symbols[0])
        },
        "type-of" (x) -> {
            l.type_name(x.what_type())
        },
        "car" (cons) -> {
            let cons = into_type_or_error!(l : cons => &mut ConsCell);
            let car: &mut Object = &mut cons.car;
            Object::from(Place::from(car as *mut Object))
        },
        "cdr" (cons) -> {
            let cons = into_type_or_error!(l : cons => &mut ConsCell);
            let cdr: &mut Object = &mut cons.cdr;
            Object::from(Place::from(cdr as *mut Object))
        },
        "nth" (list n) -> {
            let list_cons = into_type_or_error!(l : list => &mut ConsCell);
            let n_int = into_type_or_error!(l : n => i32);
            let mut iter = list_cons.into_iter();
            for _ in 0..n_int {
                if iter.next().is_none() {
                    return l.alloc(RlispError::index_out_of_bounds(n, list));
                }
            }
            if let Some(place) = iter.next() {
                Object::from(place)
            } else {
                l.alloc(RlispError::index_out_of_bounds(n, list))
            }
        },
    }
}

pub fn builtin_vars() -> RlispBuiltinVars {
    builtin_vars! {
        "+pi+" = ::std::f64::consts::PI,
        "+e+" = ::std::f64::consts::E,
        "+sqrt2+" = ::std::f64::consts::SQRT_2,

        "+ln2+" = ::std::f64::consts::LN_2,
        "+ln10+" = ::std::f64::consts::LN_10,

        "+log2-e+" = ::std::f64::consts::LOG2_E,
        "+lge+" = ::std::f64::consts::LOG10_E,

        "+1/pi+" = ::std::f64::consts::FRAC_1_PI,
        "+2/pi+" = ::std::f64::consts::FRAC_2_PI,

        "+pi/2+" = ::std::f64::consts::FRAC_PI_2,
        "+pi/3+" = ::std::f64::consts::FRAC_PI_3,
        "+pi/4+" = ::std::f64::consts::FRAC_PI_4,
        "+pi/6+" = ::std::f64::consts::FRAC_PI_6,
        "+pi/8+" = ::std::f64::consts::FRAC_PI_8,

        "+1/sqrt2+" = ::std::f64::consts::FRAC_1_SQRT_2,

        "+infinity+" = ::std::f64::INFINITY,
        "+-infinity+" = ::std::f64::NEG_INFINITY,
        "+epsilon+" = ::std::f64::EPSILON,
        "+min-num+" = ::std::f64::MIN,
        "+max-num+" = ::std::f64::MAX,
        "+nan+" = ::std::f64::NAN,
        "+min-integer+" = ::std::i32::MIN,
        "+max-integer+" = ::std::i32::MAX,
    }
}
