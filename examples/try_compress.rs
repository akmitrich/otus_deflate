fn main() {
    let msg = b"H-Code";
    let code = otus_deflate::huffman::generate::generate_fixed_code();
    println!(
        "{:?} ---> {:?}",
        msg,
        otus_deflate::deflate::deflate(msg, code)
    );
}
