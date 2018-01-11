use std::fmt;
use list;
use super::Object;

#[derive(Clone)]
pub struct ConsCell {
    pub car: Object,
    pub cdr: Object,
}

impl ConsCell {
    pub fn new(car: Object, cdr: Object) -> Self {
        Self { car: car, cdr: cdr }
    }
}

impl fmt::Display for ConsCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use list::ConsIteratorResult::*;

        write!(f, "(")?;
        let mut iter = list::iter(self);

        if let More(obj) = iter.improper_next() {
            // A list will always have a first item, so we don't need to check
            // for Final in this one
            write!(f, "{}", obj)?;
        }

        'iter: loop {
            let res = iter.improper_next();
            if let More(obj) = res {
                write!(f, " {}", obj)?;
            } else if let Final(Some(obj)) = res {
                write!(f, " . {}", obj)?;
                break 'iter;
            } else {
                break 'iter;
            }
        }

        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::ConsCell;
    use types::*;
    use list;
    #[test]
    fn display_list() {
        let my_list = list::from_vec(vec![
            Object::float(3.2),
            Object::fixnum(18),
            Object::nil(),
            Object::fixnum(0),
        ]);
        assert_eq!("(3.2 18 nil 0)", format!("{}", my_list));
    }
    #[test]
    fn display_pair() {
        let my_pair = ConsCell::new(Object::float(2.2), Object::fixnum(12));
        assert_eq!("(2.2 . 12)", format!("{}", my_pair));
    }
    #[test]
    fn display_improper_list() {
        let my_improper_list = list::improper_from_vec(vec![
            Object::float(3.2),
            Object::fixnum(18),
            Object::nil(),
            Object::fixnum(0),
        ]);
        assert_eq!("(3.2 18 nil . 0)", format!("{}", my_improper_list));
    }
}
