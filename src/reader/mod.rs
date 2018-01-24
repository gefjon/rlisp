/*
I'm not going to lie, a lot of black magic goes on in this module. The
core of it is the function `Reader::read()`, which is passed an `&mut
Iterator<u8>` and parses an object from it.xs
 */

use lisp;
use list;
use result::*;
use std::iter::{Iterator, Peekable};
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
const COMMENT_DESIGNATORS: &[u8] = &[b';'];
const COMMENT_ENDS: &[u8] = &[b'\n'];

// This trait exists because writing all of these as required traits on Reader
// makes that line so long that `cargo fmt` doesn't know what to do and errors
pub trait ReaderDepends
    : lisp::Symbols + lisp::MacroChars + lisp::allocate::AllocObject + list::ListOps
    {
}

impl ReaderDepends for lisp::Lisp {}

pub trait Reader: strings::ReadString + ReaderDepends {
    fn read<V: Iterator<Item = u8>>(&mut self, input: &mut Peekable<V>) -> Result<Option<Object>> {
        debug!("called read()");
        // This is the function called by `Rep`.
        self.read_form(input)
    }

    fn next<V: Iterator<Item = u8>>(input: &mut Peekable<V>) -> Option<u8> {
        // this method skips past comments, which it does by checking if each
        // new peek'd character is in `COMMENT_DESIGNATORS`, and then looping
        // until it hits a member of `COMMENT_ENDS`
        match input.next() {
            Some(next) if COMMENT_DESIGNATORS.contains(&next) => loop {
                match input.next() {
                    None => {
                        return None;
                    }
                    Some(next) if COMMENT_ENDS.contains(&next) => {
                        return input.next();
                    }
                    Some(_) => {
                        continue;
                    }
                }
            },
            other => {
                return other;
            }
        }
    }

    fn peek<V: Iterator<Item = u8>>(input: &mut Peekable<V>) -> Option<u8> {
        // this method skips past comments, which it does by checking if each
        // new peek'd character is in `COMMENT_DESIGNATORS`, and then looping
        // until it hits a member of `COMMENT_ENDS`

        match Self::peek_without_check_comment(input) {
            Some(peek) if COMMENT_DESIGNATORS.contains(&peek) => {
                let _ = input.next();
                loop {
                    match input.next() {
                        None => {
                            return None;
                        }
                        Some(next) if COMMENT_ENDS.contains(&next) => {
                            return Self::peek(input);
                        }
                        Some(_) => {
                            continue;
                        }
                    }
                }
            }
            other => {
                return other;
            }
        }
    }

    fn peek_without_check_comment<V>(input: &mut Peekable<V>) -> Option<u8>
    where
        V: Iterator<Item = u8>,
    {
        // this method is an ugly hack to get around the borrow checker.
        // if let Some(byte) = input.peek()
        // takes out an immutable borrow on `iter` for the lifetime of `byte`
        // which means that `iter` can't be passed by reference within the
        // block. Because `u8` is `Copy`, we can just deref the u8,
        // which is what this convenience method does.
        if let Some(peek) = input.peek() {
            Some(*peek)
        } else {
            None
        }
    }

    fn read_after_checking_macro_chars<V: Iterator<Item = u8>>(
        &mut self,
        iter: &mut Peekable<V>,
    ) -> Result<Option<Object>> {
        // Some chars ('\'', '`', ',') denote macros, which are
        // expanded at read-time into calls to `quote`, `backquote`
        // and `comma`. `read_from_char` checks those, and then calls
        // this function if it does not find a match.
        if let Some(peek) = Self::peek(iter) {
            match peek {
                b'(' => {
                    let _ = Self::next(iter);
                    Ok(Some(self.read_list(iter)?))
                }
                b'"' => Ok(Some(<Self as ReadString>::read_string(self, iter)?)),
                _ if WHITESPACE.contains(&peek) => {
                    let _ = Self::next(iter);
                    self.read_form(iter)
                }
                _ => self.read_symbol_or_number(iter),
            }
        } else {
            Ok(None)
        }
    }

    fn read_form<V: Iterator<Item = u8>>(
        &mut self,
        iter: &mut Peekable<V>,
    ) -> Result<Option<Object>> {
        if let Some(peek) = Self::peek(iter) {
            if let Some(symbol) = self.check_macro_char(peek) {
                let _ = Self::next(iter);
                if let Some(obj) = self.read_form(iter)? {
                    Ok(Some(self.list_from_vec(vec![symbol, obj])))
                } else {
                    Err(ErrorKind::UnexpectedEOF.into())
                }
            } else {
                self.read_after_checking_macro_chars(iter)
            }
        } else {
            Ok(None)
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(while_let_on_iterator))]
    fn read_list<V: Iterator<Item = u8>>(&mut self, iter: &mut Peekable<V>) -> Result<Object> {
        let mut elems = Vec::new();
        while let Some(peek) = Self::peek(iter) {
            match peek {
                b')' => {
                    let _ = Self::next(iter);
                    return Ok(self.list_from_vec(elems));
                }
                _ => {
                    if let Some(el) = self.read_form(iter)? {
                        elems.push(el);
                    } else {
                        return Err(ErrorKind::UnclosedList.into());
                    }
                }
            }
        }
        Err(ErrorKind::UnclosedList.into())
    }

    fn read_symbol_or_number<V: Iterator<Item = u8>>(
        &mut self,
        iter: &mut Peekable<V>,
    ) -> Result<Option<Object>> {
        if let Some(peek) = Self::peek(iter) {
            let _ = Self::next(iter);
            let mut sym = vec![peek];
            while let Some(peek) = Self::peek(iter) {
                match peek {
                    b')' => {
                        return Ok(Some(self.finish_symbol_or_number(sym)?));
                    }
                    _ if WHITESPACE.contains(&peek) => {
                        return Ok(Some(self.finish_symbol_or_number(sym)?));
                    }
                    _ => {
                        sym.push(Self::next(iter).unwrap());
                    }
                }
            }
            Ok(Some(self.finish_symbol_or_number(sym)?))
        } else {
            Ok(None)
        }
    }
    fn finish_symbol_or_number(&mut self, sym: Vec<u8>) -> Result<Object> {
        let sym_str = String::from_utf8(sym)?;
        if let Ok(float) = f64::from_str(&sym_str) {
            Ok(Object::from(float))
        } else {
            Ok(self.intern(sym_str))
        }
    }
}

impl Reader for ::lisp::Lisp {}
