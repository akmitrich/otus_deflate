pub mod generate;

pub type CodeType = u16;

#[derive(Debug)]
pub struct CodeNode {
    pub len: u8,
    pub code: Option<CodeType>,
}

impl CodeNode {
    pub fn new(len: u8, code: CodeType) -> Self {
        let code = if len > 0 { Some(code) } else { None };
        Self { len, code }
    }
}
