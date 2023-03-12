use super::HuffmanToken;

pub const MAX_BITS: usize = 15;

pub fn generate_code(bit_lengths: &[u8]) -> Vec<HuffmanToken> {
    let mut bl_count = [0; MAX_BITS];
    let mut next_code = [0; MAX_BITS + 1];
    calc_bl_count(bit_lengths, &mut bl_count);
    calc_first_codes(&bl_count, &mut next_code);
    let mut code = vec![];
    for len in bit_lengths {
        if *len > 0 {
            let bl_index = (*len - 1) as usize;
            code.push(HuffmanToken::new(*len, next_code[bl_index]));
            next_code[bl_index] += 1;
        }
    }
    code
}

pub fn generate_fixed_code() -> (Vec<HuffmanToken>, Vec<HuffmanToken>) {
    const LL_BIT_LENGTHS: [u8; 288] = [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8,
    ];
    const CL_BIT_LENGTH: [u8; 32] = [5; 32];
    (
        generate_code(&LL_BIT_LENGTHS),
        generate_code(&CL_BIT_LENGTH),
    )
}

fn calc_first_codes(bl_count: &[usize; MAX_BITS], next_code: &mut [u16]) {
    assert!(next_code.len() > bl_count.len());
    next_code.fill(0);
    let mut code = 0;
    for bits in 0..MAX_BITS {
        code = ((code as usize + bl_count[bits]) << 1) as u16;
        next_code[bits + 1] = code;
    }
}

fn calc_bl_count(bit_lengths: &[u8], counts: &mut [usize]) {
    counts.fill(0);
    for len in bit_lengths {
        let bl_index = *len as usize;
        assert!(bl_index <= counts.len());
        if bl_index > 0 {
            //zero bit long codes are ignored
            counts[bl_index - 1] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_as_in_rfc_1951() {
        let bit_lengths = vec![3, 3, 3, 3, 3, 2, 4, 4];
        let code = generate_code(&bit_lengths);
        assert_eq!(8, code.len());
        assert_eq!(2, code[0].token.unwrap());
        assert_eq!(3, code[1].token.unwrap());
        assert_eq!(4, code[2].token.unwrap());
        assert_eq!(5, code[3].token.unwrap());
        assert_eq!(6, code[4].token.unwrap());
        assert_eq!(0, code[5].token.unwrap());
        assert_eq!(14, code[6].token.unwrap());
        assert_eq!(15, code[7].token.unwrap());
    }

    #[test]
    fn test_calc_first_codes() {
        let mut code = [0; MAX_BITS + 1];
        calc_first_codes(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &mut code);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], code);
        calc_first_codes(&[1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &mut code);
        assert_eq!(
            [0, 2, 6, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 0],
            code
        );
        calc_first_codes(&[0, 1, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &mut code);
        assert_eq!(
            [0, 0, 2, 14, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 0],
            code
        );
    }

    #[test]
    fn test_calc_counts() {
        let mut counts = [0; MAX_BITS];
        calc_bl_count(&[], &mut counts);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], counts);
        calc_bl_count(&[1, 3, 3, 2], &mut counts);
        assert_eq!([1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], counts);
        calc_bl_count(&[15, 3, 4, 3, 3, 2, 1, 3, 4, 6], &mut counts);
        assert_eq!([1, 1, 4, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1], counts);
        calc_bl_count(&[3, 3, 3, 3, 3, 2, 4, 4], &mut counts);
        assert_eq!([0, 1, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], counts);
    }

    #[test]
    fn test_fixed_huffman_code_from_rfc_1951() {
        let bit_lengths = [
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 7, 7, 7, 7, 7,
            7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8,
        ];
        let code = generate_code(&bit_lengths);
        let code = code
            .iter()
            .filter_map(|node| node.token)
            .collect::<Vec<_>>();
        let fixed_huffman_code = vec![
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
            70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
            92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
            111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
            128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144,
            145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161,
            162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178,
            179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 400, 401, 402, 403,
            404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
            421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437,
            438, 439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454,
            455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471,
            472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488,
            489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505,
            506, 507, 508, 509, 510, 511, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 192, 193, 194, 195, 196, 197, 198, 199,
        ];
        assert_eq!(fixed_huffman_code, code);
    }
}
