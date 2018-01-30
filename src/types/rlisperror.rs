use std::{convert, fmt};
use gc::{GarbageCollected, GcMark};
use types::*;

pub struct RlispError {
    pub gc_marking: GcMark,
    pub error: RlispErrorKind,
}

impl RlispError {
    pub fn wrong_type(wanted: Object, found: Object) -> Self {
        Self::from(RlispErrorKind::wrong_type(wanted, found))
    }
    pub fn improper_list() -> Self {
        Self::from(RlispErrorKind::ImproperList)
    }
}

impl convert::From<RlispErrorKind> for RlispError {
    fn from(error: RlispErrorKind) -> Self {
        Self {
            gc_marking: 0,
            error,
        }
    }
}

impl convert::From<Error> for RlispError {
    fn from(e: Error) -> Self {
        Self {
            gc_marking: 0,
            error: RlispErrorKind::RustError(e),
        }
    }
}

impl GarbageCollected for RlispError {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, _: GcMark) {}
}

impl fmt::Display for RlispError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self.error)
    }
}

pub enum RlispErrorKind {
    WrongType {
        wanted: Object,
        found: Object,
    },
    BadArgsCount {
        min: Object,
        max: Object,
        found: Object,
    },
    ImproperList,
    RustError(Error),
}

impl RlispErrorKind {
    fn wrong_type(wanted: Object, found: Object) -> Self {
        RlispErrorKind::WrongType { wanted, found }
    }
}

impl fmt::Display for RlispErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RlispErrorKind::WrongType { wanted, found } => {
                write!(f, "expected type {} but found type {}", wanted, found)
            }
            RlispErrorKind::BadArgsCount { min, max, found } => {
                if max == Object::nil() {
                    write!(f, "wanted at least {} args but found only {}", min, found)
                } else {
                    write!(
                        f,
                        "wanted between {} and {} args but found {}",
                        min, max, found
                    )
                }
            }
            RlispErrorKind::ImproperList => {
                write!(f, "found an improper list where a proper one was expected")
            }
            RlispErrorKind::RustError(ref e) => write!(f, "INTERNAL: {}", e),
        }
    }
}
