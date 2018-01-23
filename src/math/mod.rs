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
                if let Some(cons) = nums.into_cons() {
                    for el in cons.into_iter() {
                        if !num_equals(first, el) {
                            return Ok(Object::from(false));
                        }
                    }
                }
                Object::from(true)
            },
            * (&rest nums) -> {
                let mut result = 1.0;
                if let Some(cons) = nums.into_cons() {
                    for el in cons.into_iter() {
                        result *= el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            + (&rest nums) -> {
                let mut result = 0.0;
                if let Some(cons) = nums.into_cons() {
                    for el in cons.into_iter() {
                        result += el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            - (first &rest nums) -> {
                let mut result = first.into_float_or_error()?;
                if let Some(cons) = nums.into_cons() {
                    for el in cons.into_iter() {
                        result -= el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            / (first &rest nums) -> {
                let mut result = first.into_float_or_error()?;
                if let Some(cons) = nums.into_cons() {
                    for el in cons.into_iter() {
                        result /= el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            < (lesser greater) -> {
                Object::from(num_lt(lesser, greater))
            },
            <= (lesser greater) -> {
                Object::from(num_le(lesser, greater))
            },
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

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn oddp(num: Object) -> bool {
    if let Some(num) = num.into_float() {
        (num % 2.0) == 1.0
    } else {
        false
    }
}

pub fn num_lt(first: Object, second: Object) -> bool {
    if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
        first < second
    } else {
        false
    }
}

pub fn num_le(first: Object, second: Object) -> bool {
    if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
        first <= second
    } else {
        false
    }
}
