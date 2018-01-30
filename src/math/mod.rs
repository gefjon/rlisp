use types::*;

pub mod math_builtins {
    use builtins::*;
    use super::*;
    use types::conversions::MaybeFrom;
    pub fn make_builtins() -> RlispBuiltins {
        builtin_functions!{
            l = lisp;
            "=" (first &rest nums) -> {
                let first = into_type_or_error!(l : first => f64);

                if nums == Object::nil() {
                    Object::from(true)
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);

                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop, float_cmp))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => f64);

                        if first != el {
                            return Object::nil();
                        }
                    }

                    Object::from(true)
                }
            },

            "*" (&rest nums) -> {
                if nums == Object::nil() {
                    Object::from(1.0)
                } else {
                    let mut result = 1.0;

                    let nums = into_type_or_error!(l : nums => &ConsCell);

                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => f64);
                        result *= el;
                    }

                    Object::from(result)
                }
            },

            "+" (&rest nums) -> {
                if nums == Object::nil() {
                    Object::from(0.0)
                } else {
                    let mut result = 0.0;

                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => f64);
                        result += el;
                    }
                    Object::from(result)
                }
            },

            "-" (first &rest nums) -> {
                let mut result = into_type_or_error!(l : first => f64);

                if nums.nilp() {
                    first
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => f64);
                        result -= el;
                    }
                    Object::from(result)
                }
            },

            "/" (first &rest nums) -> {
                let mut result = into_type_or_error!(l : first => f64);

                if nums.nilp() {
                    first
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums {
                        let el = into_type_or_error!(l : el => f64);
                        result /= el;
                    }
                    Object::from(result)
                }
            },
            "<" (lesser greater) -> {
                let lesser = into_type_or_error!(l : lesser => f64);
                let greater = into_type_or_error!(l : greater => f64);
                Object::from(lesser < greater)
            },
            "<=" (lesser greater) -> {
                let lesser = into_type_or_error!(l : lesser => f64);
                let greater = into_type_or_error!(l : greater => f64);
                Object::from(lesser <= greater)
            },
            ">" (greater lesser) -> {
                let lesser = into_type_or_error!(l : lesser => f64);
                let greater = into_type_or_error!(l : greater => f64);
                Object::from(greater > lesser)
            },
            ">=" (greater lesser) -> {
                let lesser = into_type_or_error!(l : lesser => f64);
                let greater = into_type_or_error!(l : greater => f64);
                Object::from(greater >= lesser)
            },
            "rem" (num divisor) -> {
                let num = into_type_or_error!(l : num => f64);
                let divisor = into_type_or_error!(l : divisor => f64);
                Object::from(num % divisor)
            },
            "mod" (num modulus) -> {
                let mut num = into_type_or_error!(l : num => f64);
                let modulus = into_type_or_error!(l : modulus => f64);
                if num < 0.0 {
                    num *= -1.0;
                }
                Object::from(num % modulus)
            },
            "trunc" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.trunc();
                debug_assert!(integerp(num));
                Object::from(num)
            },
            "floor" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.floor();
                debug_assert!(integerp(num));
                Object::from(num)
            },
            "ceil" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.ceil();
                debug_assert!(integerp(num));
                Object::from(num)
            },
            "round" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.round();
                debug_assert!(integerp(num));
                Object::from(num)
            },
            "integerp" (num) -> {
                if let Some(num) = f64::maybe_from(num) {
                    Object::from(integerp(num))
                } else {
                    Object::nil()
                }
            },
            "natnump" (num) -> {
                if let Some(num) = f64::maybe_from(num) {
                    Object::from(natnump(num))
                } else {
                    Object::nil()
                }
            },
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn integerp(num: f64) -> bool {
    (num.trunc() == num) && (num <= f64::from(::std::i32::MAX))
        && (num >= f64::from(::std::i32::MIN))
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn natnump(num: f64) -> bool {
    (num.trunc() == num) && (num <= f64::from(::std::u32::MAX))
        && (num >= f64::from(::std::u32::MIN))
}

pub fn oddp(num: isize) -> bool {
    (num % 2) != 0
}
