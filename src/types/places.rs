use types::*;
use std::{borrow, convert, fmt, ops};

pub struct Place(*mut Object);

impl borrow::Borrow<Object> for Place {
    fn borrow(&self) -> &Object {
        unsafe { &*(self.0) }
    }
}

impl borrow::BorrowMut<Object> for Place {
    fn borrow_mut(&mut self) -> &mut Object {
        unsafe { &mut *(self.0) }
    }
}

impl ops::Deref for Place {
    type Target = Object;
    fn deref(&self) -> &Object {
        unsafe { &*(self.0) }
    }
}

impl ops::DerefMut for Place {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe { &mut *(self.0) }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { write!(f, "{}", *(self.0)) }
    }
}

impl convert::From<*mut Object> for Place {
    fn from(obj: *mut Object) -> Self {
        Place(obj)
    }
}

impl fmt::Debug for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { write!(f, "[place -> {:?}]", *(self.0)) }
    }
}

impl FromUnchecked<Object> for Place {
    unsafe fn from_unchecked(obj: Object) -> Place {
        debug_assert!(obj.placep());
        Place(ObjectTag::Place.untag(obj.0) as _)
    }
}

impl FromObject for Place {
    fn rlisp_type() -> RlispType {
        RlispType::Place
    }
}

impl MaybeFrom<Object> for Place {
    fn maybe_from(obj: Object) -> Option<Place> {
        if Place::is_type(obj) {
            Some(unsafe { Place::from_unchecked(obj) })
        } else {
            None
        }
    }
}

impl convert::From<Place> for Object {
    fn from(p: Place) -> Object {
        Object(ObjectTag::Place.tag(p.0 as u64))
    }
}
