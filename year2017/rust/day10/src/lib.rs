use std::fmt::Write;

pub fn execute_round(
    list: &mut [u8],
    lengths: &[usize],
    cursor: &mut usize,
    skip_size: &mut usize,
) {
    let list_length = list.len();

    for length in lengths.iter().copied() {
        let mut to_reverse: Vec<u8> = Vec::with_capacity(length);
        for j in 0..length {
            to_reverse.push(list[(j + *cursor) % list_length])
        }
        to_reverse.reverse();
        for (j, reverse) in to_reverse.iter().enumerate() {
            list[(j + *cursor) % list_length] = *reverse;
        }

        *cursor = (*cursor + *skip_size + length) % list_length;
        *skip_size += 1;
    }
}

pub fn make_knot_hash(input: &str) -> String {
    let mut lengths_ascii: Vec<usize> = input
        .trim()
        .chars()
        .map(|a| a as u8)
        .map(|a| a as usize)
        .collect();
    lengths_ascii.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut list: Vec<u8> = (0..=255).collect();
    let mut cursor = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        execute_round(&mut list, &lengths_ascii, &mut cursor, &mut skip_size);
    }

    assert!(list.len() % 16 == 0);
    let mut elements = Vec::with_capacity(16);
    for chunk in list.chunks(16) {
        let xored = chunk.iter().fold(0, |acc, &x| acc ^ x);
        elements.push(xored)
    }
    let mut hex_string = String::new();
    for x in &elements {
        write!(&mut hex_string, "{:02x}", x).unwrap();
    }
    hex_string
}
