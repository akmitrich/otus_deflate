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

    pub fn write_bits(&mut self, n: usize, value: usize) {
        assert!(n <= mem::size_of::<usize>());
        for bit_index in (0..n).rev() {
            let bit = (value & (1 << bit_index)) > 0;
            self.write_bit(bit as _);
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
            let bit_shift = BYTE_SIZE - self.bit_pos - 1;
            self.current |= 1 << bit_shift;
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
            self.write_bits(BYTE_SIZE, *byte as _);
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
        os.write_bit(false);
        assert_eq!(0, os.current);
        assert_eq!(1, os.bit_pos);
        os.write_bits(3, 0b101);
        assert_eq!(0b0101_0000, os.current);
        assert_eq!(4, os.bit_pos);
        assert_eq!(&[0b0101_0000], os.finalize().as_slice());
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
