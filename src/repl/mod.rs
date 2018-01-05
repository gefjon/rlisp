use result::*;

use lisp::Lisp;

pub trait Rep {
    type Input;
    type Output;
    type Read;
    type Evaled;

    fn read(&self, input: Self::Input) -> Result<Self::Read>;
    fn eval(&mut self, read: Self::Read) -> Result<Self::Evaled>;
    fn print(&self, evaled: Self::Evaled) -> Result<Self::Output>;
    fn rep(&mut self, input: Self::Input) -> Result<Self::Output> {
        let read = self.read(input)?;
        let evaled = self.eval(read)?;
        self.print(evaled)
    }
}

impl Rep for Lisp {
    type Input = String;
    type Output = String;
    type Read = String;
    type Evaled = String;

    fn read(&self, input: Self::Input) -> Result<Self::Read> {
        Ok(input)
    }
    fn eval(&mut self, read: Self::Read) -> Result<Self::Evaled> {
        Ok(read)
    }
    fn print(&self, evaled: Self::Evaled) -> Result<Self::Output> {
        Ok(evaled)
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
