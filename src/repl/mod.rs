use result::*;
use lisp::Lisp;
use types::*;
use std::io;
use std::iter::{IntoIterator, Map};

pub mod stdio;

pub trait Rep<V: Iterator<Item=u8>>: ::reader::Reader<V> {
    fn read(&mut self, input: &mut V) -> Result<Option<Object>> {
        <Self as ::reader::Reader<V>>::read(self, input)
    }
    fn eval(&mut self, read: Object) -> Result<Object> {
        Ok(read)
    }
    fn print(&self, evaled: Object) -> Result<String> {
        Ok(format!("{}\n", evaled))
    }
    fn rep(&mut self, input: &mut V) -> Result<String> {
        let read = <Self as Rep<V>>::read(self, input)?;
        if let Some(obj) = read {
            let evaled = self.eval(obj)?;
            self.print(evaled)
        } else {
            Ok(String::new())
        }
    }
}

impl<'read> Rep<::reader::StdioIter<'read>> for Lisp {}

#[cfg(test)]
mod tests {
    use result::*;
    use lisp::Lisp;
    use super::Rep;
    #[test]
    fn simple_print() {
        let input = String::from("'(foo bar)");
        let mut lisp = Lisp::new();
        let output = lisp.rep(input.clone()).unwrap();
        assert_eq!(input, output);
    }
}
