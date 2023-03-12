use std::{
    io::{self, Write},
    mem,
};

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
        let os = OutputStream::new();
        assert!(os.output.is_empty());
    }

    #[test]
    fn test_write_bit() {
        let mut os = OutputStream::new();
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
        let mut os = OutputStream::new();
        let buf = [1, 2, 3];
        os.write(&buf).unwrap();
        let output = os.finalize();
        assert_eq!(buf, output.as_slice());
    }
}
