use std::io;

use crate::{bitstream::ostream::OutputStream, CodeNode};

const BUF_SIZE: usize = 4096;

pub fn deflate<R: io::Read>(mut data: R, code: Vec<CodeNode>) -> Vec<u8> {
    let mut buf = [0; BUF_SIZE];
    let mut input = vec![];
    while let Ok(size) = data.read(&mut buf) {
        if size == 0 {
            break;
        }
        input.extend_from_slice(&buf[..size]);
    }
    let encoder = Deflator::new(&input, code);
    encoder.deflate()
}

const BFINAL_YES: usize = 1;
const BTYPE_FIXED: usize = 1;
const END_OF_BLOCK: usize = 256;

#[derive(Debug)]
pub struct Deflator<'a> {
    input: &'a [u8],
    pos: usize,
    os: OutputStream,
    code: Vec<CodeNode>,
}

impl<'a> Deflator<'a> {
    fn new(input: &'a [u8], code: Vec<CodeNode>) -> Self {
        Self {
            input,
            pos: 0,
            os: OutputStream::new(),
            code,
        }
    }

    fn deflate(mut self) -> Vec<u8> {
        self.start();
        while self.pos < self.input.len() {
            self.proceed();
        }
        self.stop();
        self.finalize()
    }

    fn find_sequence(&self) -> Option<(usize, usize)> {
        None
    }

    fn encode_literal(&mut self, byte: u8) {
        if let CodeNode {
            len,
            code: Some(code),
        } = &self.code[byte as usize]
        {
            self.os.write_bits(*len as _, *code as _);
        }
    }

    fn proceed(&mut self) {
        if let Some(_) = self.find_sequence() {
        } else {
            let byte = self.input[self.pos];
            self.encode_literal(byte);
            self.pos += 1;
        }
    }

    fn start(&mut self) {
        self.os.write_bits(1, BFINAL_YES);
        self.os.write_bits(2, BTYPE_FIXED);
    }

    fn stop(&mut self) {
        if let CodeNode {
            len,
            code: Some(code),
        } = &self.code[END_OF_BLOCK]
        {
            self.os.write_bits(*len as _, *code as _);
        }
    }

    fn finalize(self) -> Vec<u8> {
        self.os.finalize()
    }
}
