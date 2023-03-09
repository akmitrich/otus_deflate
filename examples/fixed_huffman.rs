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

    let codes = otus_deflate::generate_code(&bit_lengths);

    for (index, code) in codes.iter().filter_map(|node| node.code).enumerate() {
        println!("{index}. {code:#016b}");
    }
    println!(
        "Length code:\n{}",
        (0..288)
            .filter_map(|len| otus_deflate::deflate::CONVERT_LENGTH
                .get(&len)
                .and_then(|x| Some(format!("{} => {:?}", len, x))))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
