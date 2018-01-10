use result::*;
use std::iter::{Iterator, IntoIterator};
use std::slice::Iter;
use types::*;
use lisp;
use super::WHITESPACE;

pub trait ReadSymbol<V: IntoIterator<Item=u8>>: lisp::Symbols {
    fn read_symbol(&mut self, peek: u8, iter: &mut V::IntoIter)
                   -> Result<(Object, Option<u8>)> {
        let mut sym = vec![peek];
        while let Some(byte) = iter.next() {
            match byte {
                b')' => {
                    return self.finish_symbol(sym, Some(byte));
                },
                _ if WHITESPACE.contains(&byte) => {
                    return self.finish_symbol(sym, Some(byte));
                },
                _ => sym.push(byte),
            }
        }
        self.finish_symbol(sym, None)
    }
    fn finish_symbol(&mut self, sym: Vec<u8>, end_char: Option<u8>)
                     -> Result<(Object, Option<u8>)> {
        let sym_str = String::from_utf8(sym)?;
        Ok((self.intern(sym_str).clone().into(), end_char))
    }
}

impl ReadSymbol<Vec<u8>> for lisp::Lisp {}
