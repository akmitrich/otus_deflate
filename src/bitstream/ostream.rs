use std::io;

use super::BYTE_SIZE;

#[derive(Debug, Default)]
pub struct OutputStream {
    output: Vec<u8>,
    current: u8,
    bit_pos: usize,
}

impl OutputStream {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write_bit(&mut self, bit: u8) {
        assert!(self.bit_pos < BYTE_SIZE);
        let bit = bit & 0b1;
        let bit_shift = BYTE_SIZE - self.bit_pos - 1;
        self.advance(1);
        self.current &= bit << bit_shift;
    }

    pub fn finalize(mut self) -> Vec<u8> {
        self.output.push(self.current);
        self.output
    }
}

impl OutputStream {
    fn advance(&mut self, n: usize) {
        assert!(n <= BYTE_SIZE);
        for _ in 0..n {
            self.bit_pos += 1;
            if self.bit_pos % BYTE_SIZE == 0 {
                self.output.push(self.current);
                self.bit_pos = 0;
                self.current = 0;
            }
        }
    }
}

impl io::Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

struct BitIter {
    byte: u8,
    pos: usize,
}

impl BitIter {
    pub fn new(byte: u8) -> Self {
        Self { byte, pos: 0 }
    }
}

impl Iterator for BitIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < BYTE_SIZE {
            let bit_shift = BYTE_SIZE - self.pos - 1;
            self.pos += 1;
            Some(self.byte & (1 << bit_shift) > 0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_test() {
        let os = OutputStream::new();
        assert!(os.output.is_empty());
    }

    #[test]
    fn test_write_bit() {
        let mut os = OutputStream::new();
        os.write_bit(0);
        assert_eq!(0, os.current);
        assert_eq!(1, os.bit_pos);
        assert!(!os.finalize().is_empty());
    }

    #[test]
    fn test_bit_iter() {
        let i = BitIter::new(0b01100011);
        assert_eq!(
            vec![false, true, true, false, false, false, true, true],
            i.collect::<Vec<_>>()
        );
    }
}
