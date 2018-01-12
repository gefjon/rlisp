use types::*;

pub fn improper_from_vec(mut elems: Vec<Object>) -> Object {
    if elems.is_empty() {
        Object::nil()
    } else {
        elems.reverse();
        let mut drain = elems.iter();
        let mut head = if let Some(obj) = drain.next() {
            *obj
        } else {
            return Object::nil();
        };
        for el in drain {
            head = Object::cons(*el, head);
        }
        head
    }
}

pub fn from_vec(mut elems: Vec<Object>) -> Object {
    elems.reverse();
    let mut head = Object::nil();
    for el in &elems {
        head = Object::cons(*el, head);
    }
    head
}

pub fn iter(list: &ConsCell) -> ConsIterator {
    ConsIterator {
        car: list.car,
        cdr: list.cdr,
        first: true,
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

impl ConsIterator {
    pub fn improper_next(&mut self) -> ConsIteratorResult<Object> {
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
                Object::Nil => ConsIteratorResult::Final(None),
                other => ConsIteratorResult::Final(Some(other)),
            }
        }
    }
}
