use std::{collections::HashMap, io};

use lazy_static::lazy_static;

use crate::CodeNode;

const BUF_SIZE: usize = 4096;

pub fn deflate<R: io::Read>(mut data: R, _code: Vec<CodeNode>) -> Vec<CodePoint> {
    use CodePoint::*;
    let mut buf = [0; BUF_SIZE];
    let mut input = vec![];
    while let Ok(size) = data.read(&mut buf) {
        if size == 0 {
            break;
        }
        input.extend_from_slice(&buf[..size]);
    }
    let encoder = Deflator::new(&input);
    [Bhead(BFINAL_YES), Btype(BTYPE_FIXED)]
        .into_iter()
        .chain(encoder)
        .chain([EndOfBlock].into_iter())
        .collect()
}

const BFINAL_YES: u16 = 1;
const BTYPE_FIXED: u16 = 1;
const MIN_SEQUENCE: usize = 3;

lazy_static! {
    pub static ref CONVERT_LENGTH: HashMap<usize, (usize, usize, usize)> = {
        let mut map = HashMap::new();
        for len in 3..259 {
            let entry = match len {
                len if len < 11 => (len + 254, 0, 0),
                len if len < 19 => ((len - 11) / 2 + 265, 1, (len + 1) % 2),
                len if len < 35 => ((len - 19) / 4 + 269, 2, (len + 1) % 4),
                len if len < 67 => ((len - 35) / 8 + 273, 3, (len - 3) % 8),
                len if len < 131 => ((len - 67) / 16 + 277, 4, (len - 3) % 16),
                len if len < 258 => ((len - 131) / 32 + 281, 5, (len - 3) % 32),
                258 => (285, 0, 0),
                _ => unreachable!(),
            };
            map.insert(len, entry);
        }
        map
    };
}

#[derive(Debug)]
pub struct Deflator<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Deflator<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    fn find_sequence(&self) -> Option<(u16, u16)> {
        if self.pos < MIN_SEQUENCE || self.input.len() - self.pos < MIN_SEQUENCE {
            return None;
        }
        self.lookup(3).map(|index| (3, (self.pos - index) as u16))
    }

    fn lookup(&self, length: usize) -> Option<usize> {
        let mut offset = 0;
        for index in 0..=(self.pos) {
            //look in `self.input` at every byte from the begining till last `length` byte before `self.pos`
            if length <= offset {
                return Some(index - length);
            }
            if self.input[self.pos + offset] == self.input[index] {
                offset += 1;
            } else {
                offset = 0;
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum CodePoint {
    Bhead(u16),
    Btype(u16),
    Literal(u16),
    EndOfBlock,
    Backref(u16, u16),
}

impl<'a> Iterator for Deflator<'a> {
    type Item = CodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.input.len() {
            let current = self.pos;
            if let Some((length, distance)) = self.find_sequence() {
                self.pos += length as usize;
                Some(CodePoint::Backref(length, distance))
            } else {
                self.pos += 1;
                Some(CodePoint::Literal(self.input[current] as _))
            }
        } else {
            None
        }
    }
}
