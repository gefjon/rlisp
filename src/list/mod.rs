use types::*;

pub fn improper_from_vec(mut elems: Vec<Object>) -> Object {
    if elems.len() == 0 {
        Object::nil()
    } else {
        elems.reverse();
        let mut drain = elems.drain(..);
        let mut head = if let Some(obj) = drain.next() {
            obj
        } else {
            return Object::nil();
        };
        for el in drain {
            head = Object::cons(el, head);
        }
        head
    }
}

pub fn from_vec(mut elems: Vec<Object>) -> Object {
    elems.reverse();
    let mut head = Object::nil();
    for el in elems.drain(..) {
        head = Object::cons(el, head);
    }
    head
}
pub fn iter(list: &ConsCell) -> ConsIterator {
    ConsIterator {
        car: &list.car,
        cdr: &list.cdr,
        first: true,
    }
}

pub struct ConsIterator<'cons> {
    car: &'cons Object,
    cdr: &'cons Object,
    first: bool,
}

pub enum ConsIteratorResult<'a, T: 'a> {
    More(&'a T),
    Final(Option<&'a T>),
}

impl<'cons> ConsIterator<'cons> {
    pub fn improper_next(&mut self) -> ConsIteratorResult<&'cons Object> {
        if self.first {
            self.first = false;
            ConsIteratorResult::More(&self.car)
        } else {
            match self.cdr {
                &Object::Cons(ref next) => {
                    self.car = &next.car;
                    self.cdr = &next.cdr;
                    ConsIteratorResult::More(&self.car)
                }
                &Object::Nil => ConsIteratorResult::Final(None),
                ref other => ConsIteratorResult::Final(Some(other)),
            }
        }
    }
}
