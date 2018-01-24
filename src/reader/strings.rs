use result::*;
use std::iter::{Iterator, Peekable};
use types::*;
use lisp;

#[cfg_attr(feature = "cargo-clippy", allow(while_let_on_iterator))]
pub trait ReadString: lisp::allocate::AllocObject {
    fn read_string<V: Iterator<Item = u8>>(&mut self, iter: &mut Peekable<V>) -> Result<Object> {
        if let Some(open) = iter.next() {
            let mut string = Vec::new();
            while let Some(byte) = iter.next() {
                match byte {
                    _ if byte == open => {
                        return Ok(self.alloc(RlispString::from(String::from_utf8(string)?)));
                    }
                    b'\\' => {
                        if let Some(escape) = iter.next() {
                            match escape {
                                b't' => string.push(b'\t'),
                                b'n' => string.push(b'\n'),
                                _ if escape == open => string.push(escape),
                                _ => {
                                    warn!(
                                        "Unrecognized escape character {} ({})",
                                        char::from(escape),
                                        escape
                                    );
                                    string.push(escape);
                                }
                            }
                        } else {
                            return Err(ErrorKind::UnclosedString.into());
                        }
                    }
                    _ => string.push(byte),
                }
            }
            Err(ErrorKind::UnclosedString.into())
        } else {
            unreachable!()
        }
    }
}

impl ReadString for lisp::Lisp {}
