use types::*;

pub mod math_builtins {
    use builtins::*;
    use super::*;
    pub fn make_builtins() -> RlispBuiltins {
        builtin_functions!{
            l = lisp;
            "=" (first &rest nums) -> {
                let first = into_type_or_error!(l : first => RlispNum);

                if nums.nilp() {
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
                if nums.nilp() {
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
                if nums.nilp() {
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

            "-" (&rest nums) -> {
                if nums.nilp() {
                    0.into()
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    let mut num = into_type_or_error!(l : nums.car => RlispNum);
                    if nums.cdr.nilp() {
                        Object::from(-num)
                    } else {
                        let rest = into_type_or_error!(l : nums.cdr => &ConsCell);
                        #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                        for el in rest.into_iter() {
                            let el = into_type_or_error!(l : el => RlispNum);
                            num -= el;
                        }
                        num.into()
                    }
                }
            },

            "/" (&rest nums) -> {
                if nums.nilp() {
                    1.into()
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);
                    let mut num = into_type_or_error!(l: nums.car => RlispNum);

                    if nums.cdr.nilp() {
                        Object::from(RlispNum::from(1) / num)
                    } else {
                        let rest = into_type_or_error!(l : nums.cdr => &ConsCell);
                        #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
                        for el in rest {
                            let el = into_type_or_error!(l : el => RlispNum);
                            num /= el;
                        }
                        num.into()
                    }
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
                let num = into_type_or_error!(l : num => RlispNum);
                let num = num.trunc();
                num.into()
            },
            "floor" (num) -> {
                let num = into_type_or_error!(l : num => RlispNum);
                let num = num.floor();
                num.into()
            },
            "ceil" (num) -> {
                let num = into_type_or_error!(l : num => RlispNum);
                let num = num.ceil();
                num.into()
            },
            "round" (num) -> {
                let num = into_type_or_error!(l : num => RlispNum);
                let num = num.round();
                num.into()
            },
            "flatten" (num) -> {
                let num = into_type_or_error!(l : num => RlispNum);
                num.try_flatten().into()
            },
            "abs" (n) -> {
                let num = into_type_or_error!(l : n => RlispNum);
                num.abs().into()
            },
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn integerp(num: f64) -> bool {
    num.trunc() == num
}

pub fn oddp(num: i32) -> bool {
    (num % 2) != 0
}
