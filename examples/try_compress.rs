fn main() {
    let origin = 1_u8 as usize;
    let mut result = 0_u8;
    for i in (0..8_usize).rev() {
        let bit = (origin & (1 << i)) > 0;
        print!("{}", if bit { 1 } else { 0 });
        if bit {
            result |= 1 << i;
        }
    }
    println!("\n{result:#08b}");
}
