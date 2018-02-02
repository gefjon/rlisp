/*
I'm not going to lie, a lot of black magic goes on in this module. The
core of it is the function `Reader::read()`, which is passed an `&mut
Iterator<u8>` and parses an object from it.
 */

use lisp;
use list;
use result::*;
use std::iter::{Iterator, Peekable};
use std::str::FromStr;
use types::*;

const WHITESPACE: &[u8] = &[b' ', b'\t', b'\n'];
const COMMENT_DESIGNATORS: &[u8] = &[b';'];
const COMMENT_ENDS: &[u8] = &[b'\n'];

pub trait Reader
    : lisp::Symbols + lisp::MacroChars + lisp::allocate::AllocObject + list::ListOps
    {
    fn read<V: Iterator<Item = u8>>(&mut self, input: &mut Peekable<V>) -> Result<Option<Object>> {
        debug!("called read()");
        // This is the function called by `Rep`.  Passed an &mut
        // Peekable<Iterator<Item = u8>>, it consumes the text
        // representing the first Rlisp object and returns that
        // object. Ok(None) signals that the iterator is empty (EOF).
        if let Some(peek) = Self::peek(input) {
            if let Some(symbol) = self.check_macro_char(peek) {
                let _ = Self::next(input);
                if let Some(obj) = self.read(input)? {
                    Ok(Some(self.list_from_vec(vec![symbol, obj])))
                } else {
                    Err(ErrorKind::UnexpectedEOF.into())
                }
            } else {
                self.read_after_checking_macro_chars(input)
            }
        } else {
            Ok(None)
        }
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
            other => other,
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
            other => other,
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
                b'"' => Ok(Some(self.read_string(iter)?)),
                _ if WHITESPACE.contains(&peek) => {
                    let _ = Self::next(iter);
                    self.read(iter)
                }
                _ => self.read_symbol_or_number(iter),
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
                    if let Some(el) = self.read(iter)? {
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
            Ok(Object::from(self.make_symbol(sym_str)))
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(while_let_on_iterator))]
    fn read_string<V: Iterator<Item = u8>>(&mut self, iter: &mut Peekable<V>) -> Result<Object> {
        // this method binds `open` in case in the future I decide to
        // have more characters open strings: if `%` opens a string
        // (it doesn't, but imagine it did), `"` shouldn't close it,
        // and vice versa.
        if let Some(open) = iter.next() {
            let mut string = Vec::new();

            // this method calls `iter.next()` instead of
            // `Self::next(iter)` because strings do not skip
            // comments.
            while let Some(byte) = iter.next() {
                match byte {
                    _ if byte == open => {
                        return Ok(self.alloc_string(::std::str::from_utf8(&string)?));
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

impl Reader for ::lisp::Lisp {}
