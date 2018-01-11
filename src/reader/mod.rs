use lisp;
use list;
use result::*;
use std::iter::{Iterator, Map};
use std::io;
use types::*;

mod numbers;
mod symbols;
use self::symbols::ReadSymbol;
mod strings;
use self::strings::ReadString;

const WHITESPACE: [u8; 3] = [b' ', b'\t', b'\n'];

pub type StdioIter<'read> =
    Map<io::Bytes<io::StdinLock<'read>>, fn(::std::result::Result<u8, io::Error>) -> u8>;

pub trait Reader<V>
    : numbers::ReadNumber<V>
    + strings::ReadString<V>
    + symbols::ReadSymbol<V>
    + lisp::Symbols
    + lisp::MacroChars
where
    V: Iterator<Item = u8>,
{
    fn read(&mut self, input: &mut V) -> Result<Option<Object>> {
        let (opt_form, _) = self.read_form(input)?;
        Ok(opt_form)
    }

    fn read_after_checking_macro_chars(
        &mut self,
        byte: u8,
        iter: &mut V,
    ) -> Result<(Option<Object>, Option<u8>)> {
        match byte {
            peek @ b'0'...b'9' => {
                let (obj, opt_byte) = self.read_number(peek, iter)?;
                Ok((Some(obj), opt_byte))
            }
            b'(' => Ok((Some(self.read_list(iter)?), None)),
            open @ b'"' => Ok((
                Some(<Self as ReadString<V>>::read_string(self, open, iter)?),
                None,
            )),
            _ if WHITESPACE.contains(&byte) => self.read_form(iter),
            peek => {
                let (obj, opt_byte) = <Self as ReadSymbol<V>>::read_symbol(self, peek, iter)?;
                Ok((Some(obj), opt_byte))
            }
        }
    }

    fn read_from_char(&mut self, byte: u8, iter: &mut V) -> Result<(Option<Object>, Option<u8>)> {
        let symbol = {
            if let Some(sym) = self.check_macro_char(byte) {
                Some(sym.clone())
            } else {
                None
            }
        };
        if let Some(symbol) = symbol {
            if let (Some(obj), peek) = self.read_form(iter)? {
                Ok((Some(list::from_vec(vec![symbol.into(), obj])), peek))
            } else {
                Err(ErrorKind::UnexpectedEOF.into())
            }
        } else {
            self.read_after_checking_macro_chars(byte, iter)
        }
    }

    fn read_form(&mut self, iter: &mut V) -> Result<(Option<Object>, Option<u8>)> {
        if let Some(byte) = iter.next() {
            self.read_from_char(byte, iter)
        } else {
            Ok((None, None))
        }
    }

    fn read_list(&mut self, iter: &mut V) -> Result<Object> {
        let mut elems = Vec::new();
        while let Some(byte) = iter.next() {
            match byte {
                b')' => {
                    return Ok(list::from_vec(elems));
                }
                _ => {
                    let (opt_el, opt_byte) = self.read_from_char(byte, iter)?;
                    if let Some(el) = opt_el {
                        elems.push(el);
                    } else {
                        return Err(ErrorKind::UnclosedList.into());
                    }
                    if let Some(b')') = opt_byte {
                        return Ok(list::from_vec(elems));
                    }
                }
            }
        }
        Err(ErrorKind::UnclosedList.into())
    }
}

impl<'read> Reader<StdioIter<'read>> for ::lisp::Lisp {}
