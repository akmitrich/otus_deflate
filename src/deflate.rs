use std::{collections::HashMap, io};

use lazy_static::lazy_static;

use crate::{bitstream::ostream::OutputStream, HuffmanToken};

pub fn deflate<R: io::Read>(mut data: R) -> Vec<DeflateToken> {
    use DeflateToken::*;
    let mut input = vec![];
    data.read_to_end(&mut input).unwrap();
    let encoder = Deflator::new(&input);
    [Bhead(BFINAL_YES), Btype(BTYPE_FIXED)]
        .into_iter()
        .chain(encoder)
        .chain([EndOfBlock].into_iter())
        .collect()
}

const BFINAL_YES: u16 = 1;
const BTYPE_FIXED: u16 = 1;
const END_OF_BLOCK: usize = 256;
const MIN_SEQUENCE: usize = 3;
const MAX_SEQUENCE: usize = 258;
const MAX_DISTANCE: usize = 32768;

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
    pub static ref CONVERT_DISTANCE: HashMap<usize, (usize, usize, usize)> = {
        let mut map = HashMap::new();
        for distance in 1..=MAX_DISTANCE {
            let entry = match distance {
                d if d < 5 => (d - 1, 0, 0),
                d if d < 9 => ((d - 5) / 2 + 4, 1, (d - 1) % 2),
                d if d < 17 => ((d - 9) / 4 + 6, 2, (d - 1) % 4),
                d if d < 25 => ((d - 17) / 8 + 8, 3, (d - 1) % 8),
                d if d < 65 => ((d - 25) / 16 + 10, 4, (d - 1) % 16),
                d if d < 129 => ((d - 65) / 32 + 12, 5, (d - 1) % 32),
                d if d < 257 => ((d - 129) / 64 + 14, 6, (d - 1) % 64),
                d if d < 513 => ((d - 257) / 128 + 16, 7, (d - 1) % 128),
                d if d < 1025 => ((d - 513) / 256 + 18, 8, (d - 1) % 256),
                d if d < 2049 => ((d - 1025) / 512 + 20, 9, (d - 1) % 512),
                d if d < 4097 => ((d - 2049) / 1024 + 22, 10, (d - 1) % 1024),
                d if d < 8193 => ((d - 4097) / 2048 + 24, 11, (d - 1) % 2048),
                d if d < 16385 => ((d - 8193) / 4096 + 26, 12, (d - 1) % 4096),
                d => ((d - 16385) / 8192 + 28, 13, (d - 1) % 8192),
            };
            map.insert(distance, entry);
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
        if self.input.len() - self.pos < MIN_SEQUENCE {
            return None;
        }
        let (mut index, mut len) = (0, 0);
        let start = if self.pos < MAX_DISTANCE {
            0
        } else {
            self.pos - MAX_DISTANCE
        };
        for current in (start..self.pos).rev() {
            let longest = self.lookup(current, self.pos);
            if longest > len {
                len = longest;
                index = current;
            }
        }
        if len > 2 {
            Some((len as _, (self.pos - index) as _))
        } else {
            None
        }
    }

    fn lookup(&self, x: usize, y: usize) -> usize {
        // y is self.pos
        let mut len = 0;
        let end = if y + MAX_SEQUENCE > self.input.len() {
            self.input.len() - y
        } else {
            MAX_SEQUENCE
        };
        for offset in 0..end {
            if self.input[x + offset] == self.input[y + offset] {
                len += 1;
            } else {
                break;
            }
        }
        len
    }
}

#[derive(Debug, Clone)]
pub enum DeflateToken {
    Bhead(u16),
    Btype(u16),
    Literal(u16),
    EndOfBlock,
    Backref { length: u16, distance: u16 },
}

impl DeflateToken {
    pub fn write_to_ostream(
        &self,
        ll_code: &[HuffmanToken],
        d_code: &[HuffmanToken],
        os: &mut OutputStream,
    ) {
        match self {
            DeflateToken::Bhead(head) => os.write_numerical(1, *head as _),
            DeflateToken::Btype(b_type) => os.write_numerical(2, *b_type as _),
            DeflateToken::Literal(literal) => {
                let huffman_token = &ll_code[*literal as usize];
                os.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _)
            }
            DeflateToken::EndOfBlock => {
                let huffman_token = &ll_code[END_OF_BLOCK];
                os.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _);
            }
            DeflateToken::Backref { length, distance } => {
                let (token, extra, bits) = CONVERT_LENGTH[&(*length as usize)];
                let huffman_token = &ll_code[token];
                os.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _);
                os.write_numerical(extra, bits);
                let (token, extra, bits) = CONVERT_DISTANCE[&(*distance as usize)];
                let huffman_token = &d_code[token];
                os.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _);
                os.write_numerical(extra, bits);
            }
        }
    }
}

impl<'a> Iterator for Deflator<'a> {
    type Item = DeflateToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.input.len() {
            let current = self.pos;
            if let Some((length, distance)) = self.find_sequence() {
                self.pos += length as usize;
                Some(DeflateToken::Backref { length, distance })
            } else {
                self.pos += 1;
                Some(DeflateToken::Literal(self.input[current] as _))
            }
        } else {
            None
        }
    }
}
