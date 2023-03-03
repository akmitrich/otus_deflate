use std::io;

use super::BYTE_SIZE;

#[derive(Debug)]
pub struct InputStream<'a> {
    input: &'a [u8],
    pos: usize,
    remain: usize,
}

impl<'a> InputStream<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0,
            remain: input.len() * BYTE_SIZE,
        }
    }

    pub fn bits_remain(&self) -> usize {
        self.remain
    }

    pub fn read_bit(&mut self) -> Option<u8> {
        if self.remain == 0 {
            return None;
        }
        let (byte_pos, bit_pos) = self.get_positions();
        let bit_shift = BYTE_SIZE - bit_pos - 1;
        if self.advance(1) {
            Some((self.input[byte_pos] >> bit_shift) & 1)
        } else {
            None
        }
    }
}

impl<'a> InputStream<'a> {
    fn get_positions(&self) -> (usize, usize) {
        (self.pos / BYTE_SIZE, self.pos % BYTE_SIZE)
    }

    fn advance(&mut self, n: usize) -> bool {
        if self.remain < n {
            return false;
        }
        self.remain -= n;
        self.pos += n;
        true
    }

    fn read_until_end_of_byte(&mut self) -> (u8, usize) {
        assert!(self.remain > 0);
        let (byte_pos, bit_pos) = self.get_positions();
        let last_bits = BYTE_SIZE - bit_pos;
        if self.advance(last_bits) {
            (get_last_bits(self.input[byte_pos], last_bits), last_bits)
        } else {
            (0, 0)
        }
    }
}

impl<'a> io::Read for InputStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut have_read = 0;
        for byte in buf {
            if self.bits_remain() < BYTE_SIZE {
                return Ok(have_read);
            }
            let (left, bits_read) = self.read_until_end_of_byte();
            let right = if bits_read < BYTE_SIZE {
                let (byte_pos, bit_pos) = self.get_positions();
                assert_eq!(
                    0, bit_pos,
                    "Stream must be at the 0th bit of a brand new byte."
                );
                let bits_to_read = BYTE_SIZE - bits_read;
                self.advance(bits_to_read);
                strip_last_bits(self.input[byte_pos], bits_read)
            } else {
                0
            };
            have_read += 1;
            *byte = left | right;
        }
        Ok(have_read)
    }
}

fn strip_last_bits(byte: u8, n: usize) -> u8 {
    if n >= BYTE_SIZE {
        return 0;
    }
    let bit_shift = n;
    (byte >> bit_shift) << bit_shift
}

fn get_first_bits(byte: u8, n: usize) -> u8 {
    assert!(n <= 8);
    if n == 0 {
        return 0;
    }
    let bit_shift = BYTE_SIZE - n;
    byte >> bit_shift
}

fn get_last_bits(byte: u8, n: usize) -> u8 {
    assert!(n <= 8);
    if n == 0 {
        return 0;
    }
    let bit_shift = BYTE_SIZE - n;
    (byte << bit_shift) >> bit_shift
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn trivial_test() {
        let data = [];
        let mut is = InputStream::new(&data);
        assert_eq!(0, is.bits_remain());
        let mut buf = [0; 1];
        assert_eq!(0, is.read(&mut buf).unwrap())
    }

    #[test]
    fn test_read_bit() {
        let data = [127];
        let mut is = InputStream::new(&data);
        assert_eq!(0, is.read_bit().unwrap());
        assert_eq!(1, is.read_bit().unwrap())
    }

    #[test]
    fn test_read() {
        let data = [0, 1, 2, 3, 254, 255];
        let mut is = InputStream::new(&data);
        let mut buf = [0; 3];
        assert_eq!(3, is.read(&mut buf).unwrap());
        assert_eq!([0, 1, 2], buf);
        for i in 0..6 {
            assert_eq!(0, is.read_bit().unwrap(), "MSB number '{i}' must be zero");
        }
        assert_eq!(2, is.read(&mut buf).unwrap());
        assert_eq!(255, buf[0]);
    }

    #[test]
    fn test_bit_fetch() {
        let x = 0b01100011;
        assert_eq!(0, get_first_bits(x, 0));
        assert_eq!(0, get_last_bits(x, 0));
        assert_eq!(0b01100, get_first_bits(x, 5));
        assert_eq!(0b00011, get_last_bits(x, 5));
        assert_eq!(0b01100000, strip_last_bits(x, 2));
        assert_eq!(0b01100000, strip_last_bits(x, 3));
        assert_eq!(0b01100000, strip_last_bits(x, 5));
        assert_eq!(0, strip_last_bits(x, 8));
        assert_eq!(get_first_bits(x, 3), get_last_bits(x, 3));
    }

    #[test]
    fn test_read_until_end_of_byte() {
        let data = [0b01101110, 0b00001101];
        let mut is = InputStream::new(&data);
        assert_eq!(0, is.read_bit().unwrap());
        assert_eq!(1, is.read_bit().unwrap());
        assert_eq!((0b101110, 6), is.read_until_end_of_byte());
        assert_eq!((0b00001101, 8), is.read_until_end_of_byte());
    }
}
