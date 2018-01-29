use types::*;

pub mod math_builtins {
    use builtins::*;
    use super::*;
    pub fn make_builtins() -> RlispBuiltins {
        builtin_functions!{
            l = lisp;
            "=" (first &rest nums) -> {
                let first = into_type_or_error!(l : first => f64);

                if nums == Object::nil() {
                    Object::from(true)
                } else {
                    let nums = into_type_or_error!(l : nums => &ConsCell);

                    #[cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]
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
            // "<" (lesser greater) -> {
            //     Object::from(num_lt(lesser, greater))
            // },
            // "<=" (lesser greater) -> {
            //     Object::from(num_le(lesser, greater))
            // },
            // "rem" (num modulus) -> {
            //     if let (Some(num), Some(modulus)) = (num.into_float(), modulus.into_float()) {
            //         Object::from(num % modulus)
            //     } else {
            //         Object::nil()
            //     }
            // },
            // "mod" (num modulus) -> {
            //     if let (Some(mut num), Some(modulus)) = (num.into_float(), modulus.into_float()) {
            //         while num < 0.0 {
            //             num += modulus;
            //         }
            //         Object::from(num % modulus)
            //     } else {
            //         Object::nil()
            //     }
            // },
            // "trunc" (num) -> {
            //     if let Some(num) = num.into_float() {
            //         let trunced = num.trunc();
            //         debug_assert!(integerp(trunced));
            //         Object::from(trunced)
            //     } else {
            //         Object::nil()
            //     }
            // },
            // "floor" (num) -> {
            //     if let Some(num) = num.into_float() {
            //         let floored = num.floor();
            //         debug_assert!(integerp(floored));
            //         Object::from(floored)
            //     } else {
            //         Object::nil()
            //     }
            // },
            // "ceil" (num) -> {
            //     if let Some(num) = num.into_float() {
            //         let ceiled = num.ceil();
            //         debug_assert!(integerp(ceiled));
            //         Object::from(ceiled)
            //     } else {
            //         Object::nil()
            //     }
            // },
            // "round" (num) -> {
            //     if let Some(num) = num.into_float() {
            //         let rounded = num.round();
            //         debug_assert!(integerp(rounded));
            //         Object::from(rounded)
            //     } else {
            //         Object::nil()
            //     }
            // },
            // "integerp" (num) -> {
            //     if let Some(num) = num.into_float() {
            //         if integerp(num) {
            //             Object::t()
            //         } else {
            //             Object::nil()
            //         }
            //     } else {
            //         Object::nil()
            //     }
            // },
        }
    }
}

// pub trait Math: ::lisp::symbols_table::Symbols + ::lisp::allocate::AllocObject {

//     #[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
//     pub fn num_equals(first: Object, second: Object) -> bool {

//             first == second
//         } else {
//             false
//         }
//     }

//     #[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
//     pub fn oddp(num: Object) -> bool {
//         if let Some(num) = num.into_float() {
//             (num % 2.0) != 0.0
//         } else {
//             false
//         }
//     }

//     pub fn num_lt(first: Object, second: Object) -> bool {
//         if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
//             first < second
//         } else {
//             false
//         }
//     }

//     pub fn num_le(first: Object, second: Object) -> Object {

//         first <= second
//     }
// }
#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn integerp(num: f64) -> bool {
    (num.trunc() == num) && (num <= ::std::i32::MAX as _) && (num >= ::std::i32::MIN as _)
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn natnump(num: f64) -> bool {
    (num.trunc() == num) && (num <= ::std::u32::MAX as _) && (num >= ::std::u32::MIN as _)
}

pub fn oddp(num: isize) -> bool {
    (num % 2) != 0
}
