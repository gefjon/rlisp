use types::*;
use std::{cmp, convert, ops};

#[derive(Clone, Copy)]
pub enum RlispNum {
    Int(i32),
    Float(f64),
}

impl convert::From<f64> for RlispNum {
    fn from(f: f64) -> Self {
        RlispNum::Float(f)
    }
}

impl convert::From<i32> for RlispNum {
    fn from(i: i32) -> Self {
        RlispNum::Int(i)
    }
}

impl convert::From<RlispNum> for f64 {
    fn from(n: RlispNum) -> f64 {
        match n {
            RlispNum::Int(i) => f64::from(i),
            RlispNum::Float(f) => f,
        }
    }
}

impl MaybeFrom<RlispNum> for i32 {
    fn maybe_from(n: RlispNum) -> Option<i32> {
        if let RlispNum::Int(i) = n {
            Some(i)
        } else {
            None
        }
    }
}

impl ops::Add for RlispNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(self), i32::maybe_from(rhs)) {
            RlispNum::Int(lhs + rhs)
        } else {
            RlispNum::Float(f64::from(self) + f64::from(rhs))
        }
    }
}

impl ops::AddAssign for RlispNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for RlispNum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(self), i32::maybe_from(rhs)) {
            RlispNum::Int(lhs - rhs)
        } else {
            RlispNum::Float(f64::from(self) - f64::from(rhs))
        }
    }
}

impl ops::SubAssign for RlispNum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul for RlispNum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(self), i32::maybe_from(rhs)) {
            RlispNum::Int(lhs * rhs)
        } else {
            RlispNum::Float(f64::from(self) * f64::from(rhs))
        }
    }
}

impl ops::MulAssign for RlispNum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Div for RlispNum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(self), i32::maybe_from(rhs)) {
            RlispNum::Int(lhs / rhs)
        } else {
            RlispNum::Float(f64::from(self) / f64::from(rhs))
        }
    }
}

impl ops::DivAssign for RlispNum {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl ops::Neg for RlispNum {
    type Output = Self;
    fn neg(self) -> Self {
        self * Self::from(-1)
    }
}

impl ops::Rem for RlispNum {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(self), i32::maybe_from(rhs)) {
            RlispNum::Int(lhs % rhs)
        } else {
            RlispNum::Float(f64::from(self) % f64::from(rhs))
        }
    }
}

impl ops::RemAssign for RlispNum {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl cmp::PartialEq for RlispNum {
    fn eq(&self, rhs: &Self) -> bool {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(*self), i32::maybe_from(*rhs)) {
            lhs == rhs
        } else {
            f64::from(*self) == f64::from(*rhs)
        }
    }
}

impl cmp::PartialOrd for RlispNum {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(*self), i32::maybe_from(*rhs)) {
            lhs.partial_cmp(&rhs)
        } else {
            f64::from(*self).partial_cmp(&f64::from(*rhs))
        }
    }
    fn lt(&self, rhs: &Self) -> bool {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(*self), i32::maybe_from(*rhs)) {
            lhs < rhs
        } else {
            f64::from(*self) < f64::from(*rhs)
        }
    }
    fn le(&self, rhs: &Self) -> bool {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(*self), i32::maybe_from(*rhs)) {
            lhs <= rhs
        } else {
            f64::from(*self) <= f64::from(*rhs)
        }
    }
    fn gt(&self, rhs: &Self) -> bool {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(*self), i32::maybe_from(*rhs)) {
            lhs > rhs
        } else {
            f64::from(*self) > f64::from(*rhs)
        }
    }
    fn ge(&self, rhs: &Self) -> bool {
        if let (Some(lhs), Some(rhs)) = (i32::maybe_from(*self), i32::maybe_from(*rhs)) {
            lhs >= rhs
        } else {
            f64::from(*self) >= f64::from(*rhs)
        }
    }
}

impl MaybeFrom<Object> for RlispNum {
    fn maybe_from(obj: Object) -> Option<RlispNum> {
        if obj.floatp() {
            Some(RlispNum::Float(f64::from_bits(obj.0)))
        } else if obj.integerp() {
            Some(RlispNum::Int(ObjectTag::Integer.untag(obj.0) as i32))
        } else {
            None
        }
    }
}

impl FromObject for RlispNum {
    fn rlisp_type() -> RlispType {
        RlispType::Number
    }
}
