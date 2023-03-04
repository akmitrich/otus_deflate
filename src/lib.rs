pub mod bitstream;
pub mod deflate;
pub mod huffman;

pub use huffman::generate::generate_code;
pub use huffman::CodeNode;
