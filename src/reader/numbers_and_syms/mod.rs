use std::str::{FromStr, from_utf8_unchecked};
use lisp;
use super::{peek, WHITESPACE};
use std::iter::{Iterator, Peekable};
use types::Object;

#[cfg_attr(feature = "cargo-clippy",
           allow(if_same_then_else, needless_pass_by_value, transmute_int_to_float, float_cmp,
                 cast_lossless, useless_let_if_seq, let_unit_value, unreadable_literal,
                 many_single_char_names, doc_markdown, collapsible_if))]
mod copied_from_libcore;
use self::copied_from_libcore::parse::{parse_decimal, ParseResult, Sign};
use self::copied_from_libcore::{convert, extract_sign};

fn parse_number(s: &[u8]) -> Option<Object> {
    if s.is_empty() {
        return None;
    }
    let (sign, s) = extract_sign(s);
    if s.is_empty() {
        return None;
    }

    let flt = match parse_decimal(s) {
        ParseResult::Valid(decimal) => {
            if decimal.fractional.is_empty() && decimal.exp == 0 {
                let int = i32::from_str(unsafe { from_utf8_unchecked(decimal.integral) }).unwrap();
                return Some(Object::from(match sign {
                    Sign::Positive => int,
                    Sign::Negative => -int,
                }));
            } else {
                convert(decimal).unwrap()
            }
        }
        ParseResult::ShortcutToInf => ::std::f64::INFINITY,
        ParseResult::ShortcutToZero => {
            return Some(Object::from(0i32));
        }
        ParseResult::Invalid => {
            return None;
        }
    };

    match sign {
        Sign::Positive => Some(Object::from(flt)),
        Sign::Negative => Some(Object::from(-flt)),
    }
}

pub trait ReadNumsAndSyms
    : lisp::allocate::AllocObject + ::symbols_table::SymbolLookup {
    fn read_symbol_or_number<V>(
        &mut self,
        iter: &mut Peekable<V>,
    ) -> ::result::Result<Option<Object>>
    where
        V: Iterator<Item = u8>,
    {
        if let Some(p) = peek(iter) {
            let mut sym = vec![p];
            let _ = iter.next();
            while let Some(p) = peek(iter) {
                match p {
                    b')' => {
                        return Ok(Some(self.finish_symbol_or_number(sym)));
                    }
                    _ if WHITESPACE.contains(&p) => {
                        return Ok(Some(self.finish_symbol_or_number(sym)));
                    }
                    _ => {
                        sym.push(iter.next().unwrap());
                    }
                }
            }
            Ok(Some(self.finish_symbol_or_number(sym)))
        } else {
            Ok(None)
        }
    }

    fn finish_symbol_or_number(&mut self, sym: Vec<u8>) -> Object {
        if let Some(object) = parse_number(&sym) {
            object
        } else {
            Object::from(self.make_symbol(sym.as_ref()))
        }
    }
}

impl ReadNumsAndSyms for lisp::Lisp {}
