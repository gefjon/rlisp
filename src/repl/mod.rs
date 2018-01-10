use result::*;
use lisp::Lisp;
use types::*;
use std::convert::{From};
use std::iter::IntoIterator;

pub mod stdio;

pub trait Rep<V: IntoIterator<Item=u8>>: ::reader::Reader<V> {
    type Read;
    type Evaled;

    fn read(&mut self, input: V) -> Result<Self::Read>;
    fn eval(&mut self, read: Self::Read) -> Result<Self::Evaled>;
    fn print(&self, evaled: Self::Evaled) -> Result<String>;
    fn rep(&mut self, input: V) -> Result<String> {
        let read = <Self as Rep<V>>::read(self, input)?;
        let evaled = self.eval(read)?;
        self.print(evaled)
    }
}

impl Rep<Vec<u8>> for Lisp {
    type Read = Object;
    type Evaled = Object;

    fn read(&mut self, input: Vec<u8>) -> Result<Self::Read> {
        <Self as ::reader::Reader<Vec<u8>>>::read(self, input)
    }
    fn eval(&mut self, read: Self::Read) -> Result<Self::Evaled> {
        Ok(read)
    }
    fn print(&self, evaled: Self::Evaled) -> Result<String> {
        Ok(format!("{}\n", evaled))
    }
}

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
