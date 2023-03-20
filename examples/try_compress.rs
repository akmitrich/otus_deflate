use otus_deflate::bitstream::ostream::OutputStream;

fn main() {
    let msg = b"Deflate, Hello!\n";
    let (fll_code, fcl_code) = otus_deflate::huffman::generate::generate_fixed_code();
    let block = otus_deflate::deflate(&msg[..]).collect::<Vec<_>>();
    println!("{:0x?} ---> deflate block {:?}", msg, block);
    let mut os = OutputStream::new(fll_code.clone(), fcl_code.clone());
    for token in block {
        token.write_to_ostream(&fll_code, &fcl_code, &mut os);
    }
    println!("Output stream {:x?}\n", os.finalize());

    let mut os = OutputStream::new(fll_code.clone(), fcl_code.clone());
    let la_la = b"Fa-la-la-la";
    let block = otus_deflate::deflate(&la_la[..]).collect::<Vec<_>>();
    println!("{:?} ---> {:?}", la_la, block);
    for token in block {
        token.write_to_ostream(&fll_code, &fcl_code, &mut os);
    }
    println!("Output stream {:x?}\n", os.finalize());

    let mut os = OutputStream::new(fll_code.clone(), fcl_code.clone());
    let aaa = b"aaaaaaaaaaaaaaaaaaaaa";
    let block = otus_deflate::deflate(&aaa[..]).collect::<Vec<_>>();
    println!(
        "{:?} ---> {:?} CRC = {:x}",
        aaa,
        block,
        crc32fast::hash(aaa)
    );
    os.extend(otus_deflate::deflate(&aaa[..]));
    println!("Output stream {:x?}\n", os.finalize());
}
