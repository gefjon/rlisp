use types::*;

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
pub fn num_equals(first: Object, second: Object) -> bool {
    if let (Some(first), Some(second)) = (first.into_float(), second.into_float()) {
        first == second
    } else {
        false
    }
}
