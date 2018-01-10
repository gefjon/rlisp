use result::*;
use std::str::{FromStr};
use std::iter::{Iterator, IntoIterator};
use std::slice::Iter;
use types::*;
use lisp;
use super::WHITESPACE;

pub trait ReadNumber<V: IntoIterator<Item=u8>> {
    fn read_number(&mut self, peek: u8, iter: &mut V::IntoIter)
                   -> Result<(Object, Option<u8>)> {
        let mut num = vec![peek];
        while let Some(byte) = iter.next() {
            match byte {
                b')' => {
                    return Ok((
                        Object::from(f64::from_str(&String::from_utf8(num)?)?),
                        Some(byte)
                    ));
                },
                _ if WHITESPACE.contains(&byte) => {
                    return Ok((
                        Object::from(f64::from_str(&String::from_utf8(num)?)?),
                        Some(byte)
                    ));
                },
                _ => num.push(byte),
            }
        }
        Ok((Object::from(f64::from_str(&String::from_utf8(num)?)?), None))
    }
}

impl ReadNumber<Vec<u8>> for lisp::Lisp {}
