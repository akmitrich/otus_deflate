fn main() {
    let bytes = [0x7f, 0x3e, 0x4d, 0x94];
    println!("{:x?}", u32::from_le_bytes(bytes));
}
