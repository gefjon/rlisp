/*
I'm not going to lie, a lot of black magic goes on in this module. The
core of it is the function `Reader::read()`, which is passed an `&mut
Iterator<u8>` and parses an object from it.xs
*/

use lisp;
use list;
use result::*;
use std::iter::{Iterator, Map};
use std::io;
use std::str::FromStr;
use types::*;

// reading numbers, symbols, and strings are currently stored in
// traits in submodules. I would like for read_list to be its own
// trait, but it needs to recurse on read_form, which would cause a
// circular trait dependency that makes rustc unhappy. As a possible
// change for the future, the symbol, number and string readers could
// be moved into this master trait if I deem that more convenient.
mod strings;
use self::strings::ReadString;

const WHITESPACE: &[u8] = &[b' ', b'\t', b'\n'];

// This is the type that is passed (a &mut StdioIter) by the Stdio
// REPL. It's a hell of a type, which is why this alias exists.
pub type StdioIter<'read> =
    Map<io::Bytes<io::StdinLock<'read>>, fn(::std::result::Result<u8, io::Error>) -> u8>;

pub trait Reader<V>
    : strings::ReadString<V>
    + lisp::Symbols
    + lisp::MacroChars
    + lisp::allocate::AllocObject
    + list::ListOps
where
    V: Iterator<Item = u8>,
{
    fn read(&mut self, input: &mut V) -> Result<Option<Object>> {
        // This is the function called by `Rep`.
        let (opt_form, _) = self.read_form(input)?;
        Ok(opt_form)
    }

    fn read_after_checking_macro_chars(
        &mut self,
        byte: u8,
        iter: &mut V,
    ) -> Result<(Option<Object>, Option<u8>)> {
        // Some chars ('\'', '`', ',') denote macros, which are
        // expanded at read-time into calls to `quote`, `backquote`
        // and `comma`. `read_from_char` checks those, and then calls
        // this function if it does not find a match.
        match byte {
            b'(' => Ok((Some(self.read_list(iter)?), None)),
            open @ b'"' => Ok((
                Some(<Self as ReadString<V>>::read_string(self, open, iter)?),
                None,
            )),
            _ if WHITESPACE.contains(&byte) => self.read_form(iter),
            peek => {
                let (obj, opt_byte) = self.read_symbol_or_number(peek, iter)?;
                Ok((Some(obj), opt_byte))
            }
        }
    }

    fn read_from_char(&mut self, byte: u8, iter: &mut V) -> Result<(Option<Object>, Option<u8>)> {
        // see the documentation on `read_after_checking_macro_chars`
        let symbol = {
            if let Some(sym) = self.check_macro_char(byte) {
                Some(sym)
            } else {
                None
            }
        };
        if let Some(symbol) = symbol {
            if let (Some(obj), peek) = self.read_form(iter)? {
                Ok((Some(self.list_from_vec(vec![symbol, obj])), peek))
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

    #[cfg_attr(feature = "cargo-clippy", allow(while_let_on_iterator))]
    fn read_list(&mut self, iter: &mut V) -> Result<Object> {
        let mut elems = Vec::new();
        while let Some(byte) = iter.next() {
            match byte {
                b')' => {
                    return Ok(self.list_from_vec(elems));
                }
                _ => {
                    let (opt_el, opt_byte) = self.read_from_char(byte, iter)?;
                    if let Some(el) = opt_el {
                        elems.push(el);
                    } else {
                        return Err(ErrorKind::UnclosedList.into());
                    }
                    if let Some(b')') = opt_byte {
                        return Ok(self.list_from_vec(elems));
                    }
                }
            }
        }
        Err(ErrorKind::UnclosedList.into())
    }

    fn read_symbol_or_number(&mut self, peek: u8, iter: &mut V) -> Result<(Object, Option<u8>)> {
        let mut sym = vec![peek];
        for byte in iter {
            match byte {
                b')' => {
                    return self.finish_symbol_or_number(sym, Some(byte));
                }
                _ if WHITESPACE.contains(&byte) => {
                    return self.finish_symbol_or_number(sym, Some(byte));
                }
                _ => sym.push(byte),
            }
        }
        self.finish_symbol_or_number(sym, None)
    }
    fn finish_symbol_or_number(
        &mut self,
        sym: Vec<u8>,
        end_char: Option<u8>,
    ) -> Result<(Object, Option<u8>)> {
        let sym_str = String::from_utf8(sym)?;
        if let Ok(float) = f64::from_str(&sym_str) {
            Ok((Object::from(float), end_char))
        } else {
            Ok((self.intern(sym_str), end_char))
        }
    }
}

impl<'read> Reader<StdioIter<'read>> for ::lisp::Lisp {}
