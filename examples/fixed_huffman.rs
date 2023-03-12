fn main() {
    let mut bit_lengths = vec![];
    for _ in 0..144 {
        bit_lengths.push(8);
    }
    for _ in 144..256 {
        bit_lengths.push(9);
    }
    for _ in 256..280 {
        bit_lengths.push(7);
    }
    for _ in 280..288 {
        bit_lengths.push(8);
    }
    let ll_codes = otus_deflate::generate_code(&bit_lengths);

    let d_length = [5; 32];
    let d_codes = otus_deflate::generate_code(&d_length);

    for (index, code) in ll_codes.iter().filter_map(|node| node.token).enumerate() {
        println!("{index}. {code:#x}");
    }
    for (index, code) in d_codes.iter().filter_map(|node| node.token).enumerate() {
        println!("{index}. {code:#x}");
    }
    let length_ct = otus_deflate::deflate::CONVERT_LENGTH.clone();
    let distance_ct = otus_deflate::deflate::CONVERT_DISTANCE.clone();
    println!("\nLength conversion table:");
    for len in 3_usize..258 {
        let (code, extra, bits) = length_ct[&len];
        println!("{len} => {code} and {bits} in {extra} bits.")
    }
    println!("\nDistance conversion table:");
    for distance in 1_usize..=32768 {
        let (code, extra, bits) = distance_ct[&distance];
        println!("{distance} => {code} and {bits} in {extra} bits.")
    }
}
