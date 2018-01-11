use result::*;
use std::iter::Iterator;
use types::*;
use lisp;

#[cfg_attr(feature = "cargo-clippy", allow(while_let_on_iterator))]
pub trait ReadString<V: Iterator<Item = u8>>: lisp::Store<String> {
    fn read_string(&mut self, open: u8, iter: &mut V) -> Result<Object> {
        let mut string = Vec::new();
        while let Some(byte) = iter.next() {
            match byte {
                _ if byte == open => {
                    return Ok(self.store(String::from_utf8(string)?));
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
    }
}

impl<'read> ReadString<super::StdioIter<'read>> for lisp::Lisp {}
