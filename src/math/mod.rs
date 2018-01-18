use types::*;

pub mod math_builtins {
    use builtins::*;
    use list::ListOps;
    use list;
    use super::*;
    pub fn make_builtins() -> RlispBuiltins {
        builtin_functions!{
            l = lisp;
            = (first &rest nums) -> {
                let mut result = true;
                if let Some(cons) = nums.into_cons() {
                    for el in cons.into_iter() {
                        result &= num_equals(first, nums);
                    }
                }
                Object::from(result)
            },
            * (&rest nums) -> {
                let mut result = 1.0;
                if let Some(cons) = nums.into_cons() {
                    for el in cons.into_iter() {
                        result *= el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            }
        }
    }
}
#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn num_equals(first: Object, second: Object) -> bool {
    if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
        first == second
    } else {
        false
    }
}

pub fn multiply(first: Object, second: Object) -> Object {
    if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
        Object::from(first * second)
    } else {
        Object::nil()
    }
}

pub fn add(first: Object, second: Object) -> Object {
    if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
        Object::from(first + second)
    } else {
        Object::nil()
    }
}

pub fn num_lt(first: Object, second: Object) -> bool {
    if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
        first < second
    } else {
        false
    }
}
