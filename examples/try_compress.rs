use otus_deflate::bitstream::ostream::OutputStream;

fn main() {
    let msg = b"Deflate, Hello!\n";
    let (fll_code, fcl_code) = otus_deflate::huffman::generate::generate_fixed_code();
    let block = otus_deflate::deflate(&msg[..]);
    println!("{:0x?} ---> deflate block {:?}", msg, block);
    let mut os = OutputStream::new();
    for token in block {
        token.write_to_ostream(&fll_code, &fcl_code, &mut os);
    }
    println!("Output stream {:x?}\n", os.finalize());

    let mut os = OutputStream::new();
    let la_la = b"Fa-la-la-la";
    let block = otus_deflate::deflate(&la_la[..]);
    println!("{:?} ---> {:?}", la_la, block);
    for token in block {
        token.write_to_ostream(&fll_code, &fcl_code, &mut os);
    }
    println!("Output stream {:x?}\n", os.finalize());

    let mut os = OutputStream::new();
    let aaa = b"aaaaaaaaaaaaaaaaaaaaa";
    let block = otus_deflate::deflate(&aaa[..]);
    println!(
        "{:?} ---> {:?} CRC = {:x}",
        aaa,
        block,
        crc32fast::hash(aaa)
    );
    for token in block {
        token.write_to_ostream(&fll_code, &fcl_code, &mut os);
    }
    println!("Output stream {:x?}\n", os.finalize());
}
