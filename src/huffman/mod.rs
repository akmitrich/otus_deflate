pub mod generate;

#[derive(Debug)]
pub struct CodeNode {
    pub len: u8,
    pub code: Option<u16>,
}

impl CodeNode {
    pub fn new(len: u8, code: u16) -> Self {
        let code = if len > 0 { Some(code) } else { None };
        Self { len, code }
    }
}
