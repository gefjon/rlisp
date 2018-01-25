use types::*;

pub mod math_builtins {
    use builtins::*;
    use super::*;
    pub fn make_builtins() -> RlispBuiltins {
        builtin_functions!{
            l = lisp;
            "=" (first &rest nums) -> {
                if let Some(cons) = nums.into_cons() {
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in cons.into_iter() {
                        if !num_equals(first, el) {
                            return Ok(Object::from(false));
                        }
                    }
                }
                Object::from(true)
            },
            "*" (&rest nums) -> {
                let mut result = 1.0;
                if let Some(cons) = nums.into_cons() {
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in cons.into_iter() {
                        result *= el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            "+" (&rest nums) -> {
                let mut result = 0.0;
                if let Some(cons) = nums.into_cons() {
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in cons.into_iter() {
                        result += el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            "-" (first &rest nums) -> {
                let mut result = first.into_float_or_error()?;
                if let Some(cons) = nums.into_cons() {
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in cons.into_iter() {
                        result -= el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            "/" (first &rest nums) -> {
                let mut result = first.into_float_or_error()?;
                if let Some(cons) = nums.into_cons() {
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in cons.into_iter() {
                        result /= el.into_float_or_error()?;
                    }
                }
                Object::from(result)
            },
            "<" (lesser greater) -> {
                Object::from(num_lt(lesser, greater))
            },
            "<=" (lesser greater) -> {
                Object::from(num_le(lesser, greater))
            },
            "rem" (num modulus) -> {
                if let (Some(num), Some(modulus)) = (num.into_float(), modulus.into_float()) {
                    Object::from(num % modulus)
                } else {
                    Object::nil()
                }
            },
            "mod" (num modulus) -> {
                if let (Some(mut num), Some(modulus)) = (num.into_float(), modulus.into_float()) {
                    while num < 0.0 {
                        num += modulus;
                    }
                    Object::from(num % modulus)
                } else {
                    Object::nil()
                }
            },
            "trunc" (num) -> {
                if let Some(num) = num.into_float() {
                    let trunced = num.trunc();
                    debug_assert!(integerp(trunced));
                    Object::from(trunced)
                } else {
                    Object::nil()
                }
            },
            "floor" (num) -> {
                if let Some(num) = num.into_float() {
                    let floored = num.floor();
                    debug_assert!(integerp(floored));
                    Object::from(floored)
                } else {
                    Object::nil()
                }
            },
            "ceil" (num) -> {
                if let Some(num) = num.into_float() {
                    let ceiled = num.ceil();
                    debug_assert!(integerp(ceiled));
                    Object::from(ceiled)
                } else {
                    Object::nil()
                }
            },
            "round" (num) -> {
                if let Some(num) = num.into_float() {
                    let rounded = num.round();
                    debug_assert!(integerp(rounded));
                    Object::from(rounded)
                } else {
                    Object::nil()
                }
            },
            "integerp" (num) -> {
                if let Some(num) = num.into_float() {
                    if integerp(num) {
                        Object::t()
                    } else {
                        Object::nil()
                    }
                } else {
                    Object::nil()
                }
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
        (num % 2.0) != 0.0
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

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
fn integerp(num: f64) -> bool {
    num.trunc() == num
}
