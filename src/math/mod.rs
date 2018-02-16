use types::*;

pub mod math_builtins {
    use builtins::*;
    use super::*;
    use types::conversions::MaybeFrom;
    pub fn make_builtins() -> RlispBuiltins {
        builtin_functions!{
            l = lisp;
            "=" (first &rest nums) -> {
                let first = into_type_or_error!(l : first => RlispNum);

                if nums == Object::nil() {
                    true.into()
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);

                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop, float_cmp))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => RlispNum);

                        if first != el {
                            return false.into();
                        }
                    }

                    true.into()
                }
            },

            "*" (&rest nums) -> {
                if nums == Object::nil() {
                    1.into()
                } else {
                    let mut result = RlispNum::from(1);

                    let nums = into_type_or_error!(l : nums => &ConsCell);

                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => RlispNum);
                        result *= el;
                    }

                    result.into()
                }
            },

            "+" (&rest nums) -> {
                if nums == Object::nil() {
                    0.into()
                } else {
                    let mut result = RlispNum::from(0);

                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => RlispNum);
                        result += el;
                    }
                    result.into()
                }
            },

            "-" (first &rest nums) -> {
                let mut result = into_type_or_error!(l : first => RlispNum);

                if nums.nilp() {
                    first
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums.into_iter() {
                        let el = into_type_or_error!(l : el => RlispNum);
                        result -= el;
                    }
                    result.into()
                }
            },

            "/" (first &rest nums) -> {
                let mut result = into_type_or_error!(l : first => RlispNum);

                if nums.nilp() {
                    first
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                    for el in nums {
                        let el = into_type_or_error!(l : el => RlispNum);
                        result /= el;
                    }
                    result.into()
                }
            },
            "<" (lesser greater) -> {
                let lesser = into_type_or_error!(l : lesser => RlispNum);
                let greater = into_type_or_error!(l : greater => RlispNum);
                (lesser < greater).into()
            },
            "<=" (lesser greater) -> {
                let lesser = into_type_or_error!(l : lesser => RlispNum);
                let greater = into_type_or_error!(l : greater => RlispNum);
                (lesser <= greater).into()
            },
            ">" (greater lesser) -> {
                let lesser = into_type_or_error!(l : lesser => RlispNum);
                let greater = into_type_or_error!(l : greater => RlispNum);
                (greater > lesser).into()
            },
            ">=" (greater lesser) -> {
                let lesser = into_type_or_error!(l : lesser => RlispNum);
                let greater = into_type_or_error!(l : greater => RlispNum);
                (greater >= lesser).into()
            },
            "rem" (num divisor) -> {
                let num = into_type_or_error!(l : num => RlispNum);
                let divisor = into_type_or_error!(l : divisor => RlispNum);
                (num % divisor).into()
            },
            "mod" (num modulus) -> {
                let mut num = into_type_or_error!(l : num => RlispNum);
                let modulus = into_type_or_error!(l : modulus => RlispNum);
                if num < RlispNum::from(1) {
                    num *= RlispNum::from(-1);
                }
                (num % modulus).into()
            },
            "trunc" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.trunc();
                debug_assert!(integerp(num));
                num.into()
            },
            "floor" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.floor();
                debug_assert!(integerp(num));
                num.into()
            },
            "ceil" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.ceil();
                debug_assert!(integerp(num));
                num.into()
            },
            "round" (num) -> {
                let num = into_type_or_error!(l : num => f64);
                let num = num.round();
                debug_assert!(integerp(num));
                num.into()
            },
            "natnump" (num) -> {
                if let Some(num) = f64::maybe_from(num) {
                    natnump(num).into()
                } else {
                    false.into()
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

pub fn oddp(num: i32) -> bool {
    (num % 2) != 0
}
