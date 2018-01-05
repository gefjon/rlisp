use list;
use result::*;
use std::str::Bytes;
use types::*;

pub trait Reader {
    fn read(&mut self, input: String) -> Result<Object>;
    fn read_from_char(&mut self, byte: u8, iter: &mut Bytes)
                      -> Result<Option<Object>>;
    fn read_form(&mut self, iter: &mut Bytes) -> Result<Option<Object>>;
    fn read_number(&mut self, peek: u8, iter: &mut Bytes) -> Result<Object>;
    fn read_list(&mut self, iter: &mut Bytes) -> Result<Object>;
    fn read_string(&mut self, open: u8, iter: &mut Bytes) -> Result<Object>;
    fn read_symbol(&mut self, peek: u8, iter: &mut Bytes) -> Result<Object>;
}

const WHITESPACE: [u8; 3] = [b' ', b'\t', b'\n'];

impl Reader for ::lisp::Lisp {
    fn read(&mut self, input: String) -> Result<Object> {
        let iter = &mut input.bytes();
        
        let mut top_level_forms: Vec<Object> = vec![
            self.symbols.intern_str("progn").clone().into()
        ];
        while let Some(form) = self.read_form(iter)? {
            top_level_forms.push(form);
        }
        Ok(list::from_vec(top_level_forms))
    }
    fn read_from_char(&mut self, byte: u8, iter: &mut Bytes)
                      -> Result<Option<Object>> {
        match byte {
                peek @ b'0' ... b'9' => Ok(Some(self.read_number(peek, iter)?)),
                b'(' => Ok(Some(self.read_list(iter)?)),
                open @ b'"' | open @ b'\'' => Ok(Some(self.read_string(open, iter)?)),
                _ if WHITESPACE.contains(&byte) => self.read_form(iter),
                peek => Ok(Some(self.read_symbol(peek, iter)?)),
        }
    }
    fn read_form(&mut self, iter: &mut Bytes) -> Result<Option<Object>> {
        if let Some(byte) = iter.next() {
            self.read_from_char(byte, iter)
        } else {
            Ok(None)
        }
    }
    fn read_list(&mut self, iter: &mut Bytes) -> Result<Object> {
        let mut elems = Vec::new();
        while let Some(byte) = iter.next() {
            match byte {
                b')' => {
                    return Ok(list::from_vec(elems));
                },
                _ => {
                    if let Some(el) = self.read_from_char(byte, iter)? {
                        elems.push(el);
                    } else {
                        return Err(ErrorKind::UnclosedList.into());
                    }
                }
            }
        }
        Ok(list::from_vec(elems))
    }
    fn read_number(&mut self, peek: u8, iter: &mut Bytes) -> Result<Object> {
        unimplemented!()
    }
    fn read_string(&mut self, open: u8, iter: &mut Bytes) -> Result<Object> {
        unimplemented!()
    }
    fn read_symbol(&mut self, peek: u8, iter: &mut Bytes) -> Result<Object> {
        unimplemented!()
    }
}
