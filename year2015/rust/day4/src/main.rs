fn main() {
    let key = "iwrupvqb";

    let part_1 = find_noun(key, "00000");

    println!("Part 1: {part_1}");

    let part_2 = find_noun(key, "000000");

    println!("Part 2: {part_2}");
}

fn find_noun(key: &str, starts_with: &str) -> u32 {
    let mut i = 0;
    loop {
        let input = format!("{key}{i}");
        let digest = md5::compute(input);
        let hex = format!("{:x}", digest);
        let five_zeros = hex.starts_with(starts_with);
        if five_zeros {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let inputs = Vec::from([("abcdef", 609043), ("pqrstuv", 1048970)]);

        for input in inputs {
            let key = input.0;
            let expected = input.1;

            // Act
            let noun = find_noun(key, "00000");

            // Assert
            assert_eq!(noun, expected);
        }
    }
}
