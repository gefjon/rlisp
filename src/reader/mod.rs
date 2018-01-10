use lisp;
use list;
use result::*;
use std::iter::{Iterator, IntoIterator};
use std::slice::Iter;
use types::*;

mod numbers;
mod symbols;
use self::symbols::ReadSymbol;
mod strings;
use self::strings::ReadString;

const WHITESPACE: [u8; 3] = [b' ', b'\t', b'\n'];


pub trait Reader<V>: numbers::ReadNumber<V> +
    strings::ReadString<V> +
    symbols::ReadSymbol<V> +
    lisp::Symbols
    where V: IntoIterator<Item=u8>
{
    fn read(&mut self, input: V) -> Result<Object> {
        let iter = &mut input.into_iter();
        
        let mut top_level_forms: Vec<Object> = Vec::new();
        while let (Some(form), _) = self.read_form(iter)? {
            top_level_forms.push(form);
        }
        Ok(list::from_vec(top_level_forms))
    }
    
    fn read_from_char(&mut self, byte: u8, iter: &mut V::IntoIter)
                      -> Result<(Option<Object>, Option<u8>)> {
        match byte {
            peek @ b'0' ... b'9' => {
                let (obj, opt_byte) = self.read_number(peek, iter)?;
                Ok((Some(obj), opt_byte))
            },
            b'(' => Ok((Some(self.read_list(iter)?), None)),
            open @ b'"' => Ok((Some(
                <Self as ReadString<V>>::read_string(self, open, iter)?
            ), None)),
            _ if WHITESPACE.contains(&byte) => self.read_form(iter),
            peek => {
                let (obj, opt_byte) = <Self as ReadSymbol<V>>::read_symbol(
                    self,
                    peek,
                    iter
                )?;
                Ok((Some(obj), opt_byte))
            },
        }
    }
    
    fn read_form(&mut self, iter: &mut V::IntoIter)
                    -> Result<(Option<Object>, Option<u8>)> {
        if let Some(byte) = iter.next() {
            self.read_from_char(byte, iter)
        } else {
            Ok((None, None))
        }
    }
    
    fn read_list(&mut self, iter: &mut V::IntoIter) -> Result<Object> {
        let mut elems = Vec::new();
        while let Some(byte) = iter.next() {
            match byte {
                b')' => {
                    return Ok(list::from_vec(elems));
                },
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

impl Reader<Vec<u8>> for ::lisp::Lisp {}
