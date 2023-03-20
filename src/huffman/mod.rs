pub mod generate;

#[derive(Debug, Clone, Copy)]
pub struct HuffmanToken {
    pub len: u8,
    pub token: Option<u16>,
}

impl HuffmanToken {
    pub fn new(len: u8, code: u16) -> Self {
        let token = if len > 0 { Some(code) } else { None };
        Self { len, token }
    }
}
