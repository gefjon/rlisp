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
    pub fn unbound_symbol(sym: Object) -> Self {
        Self::from(RlispErrorKind::UnboundSymbol { sym })
    }
    pub fn bad_args_count(found: Object, min: Object, max: Object) -> Self {
        Self::from(RlispErrorKind::BadArgsCount { found, min, max })
    }
    pub fn not_a_type(found: Object) -> Self {
        Self::from(RlispErrorKind::NotAType { found })
    }
    pub fn undefined_symbol(sym: Object) -> Self {
        Self::from(RlispErrorKind::UndefinedSymbol { sym })
    }
    pub fn index_out_of_bounds(idx: Object, reciever: Object) -> Self {
        Self::from(RlispErrorKind::IndexOutOfBounds { idx, reciever })
    }
    pub fn custom(kind: Object, info: Object) -> Self {
        Self::from(RlispErrorKind::Custom { kind, info })
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
    UnboundSymbol {
        sym: Object,
    },
    RustError(Error),
    NotAType {
        found: Object,
    },
    Custom {
        kind: Object,
        info: Object,
    },
    UndefinedSymbol {
        sym: Object,
    },
    IndexOutOfBounds {
        idx: Object,
        reciever: Object,
    },
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
            RlispErrorKind::UnboundSymbol { sym } => write!(f, "symbol {} is unbound", sym),
            RlispErrorKind::RustError(ref e) => write!(f, "INTERNAL: {}", e),
            RlispErrorKind::NotAType { found } => write!(f, "{} is not a type designator", found),
            RlispErrorKind::Custom { kind, info } => write!(f, "{}: {}", kind, info),
            RlispErrorKind::UndefinedSymbol { sym } => write!(f, "symbol {} is undefined", sym),
            RlispErrorKind::IndexOutOfBounds { idx, reciever } => {
                write!(f, "{} is not a valid index into {}", idx, reciever)
            }
        }
    }
}

impl FromUnchecked<Object> for *mut RlispError {
    unsafe fn from_unchecked(obj: Object) -> *mut RlispError {
        debug_assert!(obj.errorp());
        ObjectTag::Error.untag(obj.0) as *mut RlispError
    }
}

impl FromObject for *mut RlispError {
    fn rlisp_type() -> RlispType {
        RlispType::Error
    }
}
