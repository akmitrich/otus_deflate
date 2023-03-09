fn main() {
    // let msg = b"H-Code";
    // let code = otus_deflate::huffman::generate::generate_fixed_code();
    // println!(
    //     "{:?} ---> {:?}",
    //     msg,
    //     otus_deflate::deflate::deflate(&msg[..], code)
    // );

    let la_la = b"Fa-la-la-la";
    let code = otus_deflate::huffman::generate::generate_fixed_code();
    println!(
        "{:?} ---> {:?}",
        la_la,
        otus_deflate::deflate::deflate(&la_la[..], code)
    );
}
