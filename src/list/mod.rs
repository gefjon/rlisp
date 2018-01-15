/*
This module includes several functions which operate on lists (chains
of conses). Those that require allocation (constructing lists, pushing
to the head of lists (and popping, just for symmetry), etc.) are
enclosed in the trait ListOps. `ConsCell`s can also be turned into a
pseudo-Iterator using list::iter(ConsCell).
*/
use types::*;
use lisp;
use std::iter::{IntoIterator, Iterator};

pub trait ListOps: lisp::allocate::AllocObject {
    fn list_improper_from_vec(&mut self, mut elems: Vec<Object>) -> Object {
        if elems.is_empty() {
            Object::nil()
        } else {
            elems.reverse();
            let mut drain = elems.iter();
            let mut head = if let Some(obj) = drain.next() {
                *obj
            } else {
                unreachable!()
            };
            for el in drain {
                head = self.alloc(ConsCell::new(*el, head));
            }
            head
        }
    }

    fn list_from_vec(&mut self, mut elems: Vec<Object>) -> Object {
        elems.reverse();
        let mut head = Object::nil();
        for el in &elems {
            head = self.alloc(ConsCell::new(*el, head));
        }
        head
    }
    fn list_push(&mut self, list: &mut Object, new_head: Object) {
        *list = self.alloc(ConsCell::new(new_head, *list));
    }
    fn list_pop(&mut self, list: &mut Object) -> Object {
        if let Some(&ConsCell { car, cdr, .. }) = list.into_cons() {
            *list = cdr;
            car
        } else {
            *list
        }
    }
}

impl ListOps for lisp::Lisp {}

pub fn iter(list: &ConsCell) -> ConsIterator {
    ConsIterator {
        car: list.car,
        cdr: list.cdr,
        first: true,
    }
}

impl<'a> IntoIterator for &'a ConsCell {
    type IntoIter = ConsIterator;
    type Item = Object;
    fn into_iter(self) -> Self::IntoIter {
        iter(self)
    }
}

pub struct ConsIterator {
    car: Object,
    cdr: Object,
    first: bool,
}

pub enum ConsIteratorResult<T> {
    More(T),
    Final(Option<T>),
}

impl Iterator for ConsIterator {
    // this implementation of Iterator is not safe for improper (not
    // nil-terminated) lists - it will discard the final element of
    // the list, nil or otherwise. For cases where an improper list is
    // a reasonable input (or where an improper list should signal an
    // error), use improper_next() instead.
    type Item = Object;
    fn next(&mut self) -> Option<Object> {
        let res = self.improper_next();
        if let ConsIteratorResult::More(obj) = res {
            Some(obj)
        } else {
            if let ConsIteratorResult::Final(Some(_)) = res {
                warn!("used Iterator on an improper list -- use ConsIterator.improper_next() instead!");
            }
            None
        }
    }
}

impl ConsIterator {
    pub fn improper_next(&mut self) -> ConsIteratorResult<Object> {
        // this version of next() is safe for improper (not
        // nil-terminated) lists - the Iterator impl will discard the
        // last element, whereas improper_next() will return it if it
        // exists, or None if the list is nil-terminated
        if self.first {
            self.first = false;
            ConsIteratorResult::More(self.car)
        } else {
            match self.cdr {
                Object::Cons(next) => {
                    self.car = unsafe { (*next).car };
                    self.cdr = unsafe { (*next).cdr };
                    ConsIteratorResult::More(self.car)
                }
                Object::Bool(false) => ConsIteratorResult::Final(None),
                other => ConsIteratorResult::Final(Some(other)),
            }
        }
    }
}
