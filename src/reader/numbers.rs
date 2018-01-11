use result::*;
use std::str::FromStr;
use std::iter::Iterator;
use types::*;
use lisp;
use super::WHITESPACE;

pub trait ReadNumber<V: Iterator<Item = u8>> {
    fn read_number(&mut self, peek: u8, iter: &mut V) -> Result<(Object, Option<u8>)> {
        let mut num = vec![peek];
        for byte in iter {
            match byte {
                b')' => {
                    return Ok((
                        Object::from(f64::from_str(&String::from_utf8(num)?)?),
                        Some(byte),
                    ));
                }
                _ if WHITESPACE.contains(&byte) => {
                    return Ok((
                        Object::from(f64::from_str(&String::from_utf8(num)?)?),
                        Some(byte),
                    ));
                }
                _ => num.push(byte),
            }
        }
        Ok((Object::from(f64::from_str(&String::from_utf8(num)?)?), None))
    }
}

impl<'read> ReadNumber<super::StdioIter<'read>> for lisp::Lisp {}
