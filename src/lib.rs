pub mod bitstream;
pub mod deflate;
pub mod huffman;

pub use deflate::deflate;
pub use huffman::generate::generate_code;
pub use huffman::HuffmanToken;
