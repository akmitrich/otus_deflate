use std::{
    io::{self, Write},
    mem,
};

use crate::{
    deflate::{DeflateToken, CONVERT_DISTANCE, CONVERT_LENGTH, END_OF_BLOCK},
    HuffmanToken,
};

use super::BYTE_SIZE;

#[derive(Debug, Default)]
pub struct OutputStream {
    output: Vec<u8>,
    current: u8,
    bit_pos: usize,
    ll_code: Vec<HuffmanToken>,
    d_code: Vec<HuffmanToken>,
}

impl OutputStream {
    pub fn new(ll_code: Vec<HuffmanToken>, d_code: Vec<HuffmanToken>) -> Self {
        Self {
            output: vec![],
            current: 0,
            bit_pos: 0,
            ll_code,
            d_code,
        }
    }

    pub fn extend(&mut self, tokens: impl Iterator<Item = DeflateToken>) {
        for ref token in tokens {
            self.write_token(token);
        }
    }

    pub fn write_token(&mut self, token: &DeflateToken) {
        match token {
            DeflateToken::Bhead(head) => self.write_numerical(1, *head as _),
            DeflateToken::Btype(b_type) => self.write_numerical(2, *b_type as _),
            DeflateToken::Literal(literal) => {
                let huffman_token = &self.ll_code[*literal as usize];
                self.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _)
            }
            DeflateToken::EndOfBlock => {
                let huffman_token = &self.ll_code[END_OF_BLOCK];
                self.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _);
            }
            DeflateToken::Backref { length, distance } => {
                let (token, extra, bits) = CONVERT_LENGTH[&(*length as usize)];
                let huffman_token = &self.ll_code[token];
                self.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _);
                self.write_numerical(extra, bits);
                let (token, extra, bits) = CONVERT_DISTANCE[&(*distance as usize)];
                let huffman_token = &self.d_code[token];
                self.write_code(huffman_token.len as _, huffman_token.token.unwrap() as _);
                self.write_numerical(extra, bits);
            }
        }
    }

    pub fn write_code(&mut self, len: usize, token: usize) {
        assert!(len <= mem::size_of::<usize>());
        for bit_index in (0..len).rev() {
            let bit = (token & (1 << bit_index)) > 0;
            self.write_bit(bit);
        }
    }

    pub fn write_numerical(&mut self, n: usize, value: usize) {
        let mut x = value;
        for _ in 0..n {
            let bit = x & 1 > 0;
            self.write_bit(bit);
            x >>= 1;
        }
    }

    pub fn finalize(mut self) -> Vec<u8> {
        if self.bit_pos % BYTE_SIZE > 0 {
            self.flush().unwrap();
        }
        self.output
    }
}

impl OutputStream {
    fn write_bit(&mut self, bit: bool) {
        assert!(self.bit_pos < BYTE_SIZE);
        if bit {
            self.current |= 1 << self.bit_pos;
        }
        self.advance(1);
    }

    fn advance(&mut self, n: usize) {
        assert!(n <= BYTE_SIZE);
        for _ in 0..n {
            self.bit_pos += 1;
            if self.bit_pos % BYTE_SIZE == 0 {
                self.flush().unwrap();
            }
        }
    }
}

impl io::Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut have_written = 0;
        for byte in buf {
            self.write_numerical(BYTE_SIZE, *byte as _);
            have_written += 1;
        }
        Ok(have_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.push(self.current);
        self.bit_pos = 0;
        self.current = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn trivial_test() {
        let os = OutputStream::default();
        assert!(os.output.is_empty());
    }

    #[test]
    fn test_write_bit() {
        let mut os = OutputStream::default();
        os.write_bit(true);
        assert_eq!(0b0000_0001, os.current);
        assert_eq!(1, os.bit_pos);
        os.write_code(3, 0b110);
        assert_eq!(0b0000_0111, os.current);
        assert_eq!(4, os.bit_pos);
        os.write_bit(false);
        assert_eq!(0b0000_0111, os.current);
        assert_eq!(5, os.bit_pos);
        assert_eq!(&[0b0000_0111], os.finalize().as_slice());
    }

    #[test]
    fn test_write_trait() {
        let mut os = OutputStream::default();
        let buf = [1, 2, 3];
        os.write(&buf).unwrap();
        let output = os.finalize();
        assert_eq!(buf, output.as_slice());
    }
}
