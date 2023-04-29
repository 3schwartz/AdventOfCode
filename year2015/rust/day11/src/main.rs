fn main() {
    let part_1 = find_password("hepxcrrq");

    println!("Part 1: {}", part_1);

    let part_2 = find_password(&part_1);

    println!("Part 2: {}", part_2);
}

fn find_password(old_password: &str) -> String {
    let mut chars = old_password.chars().collect::<Vec<char>>();

    loop {
        chars = increment(chars);

        if rule_two_pairs(&chars)
            && rule_increasing_straight(&chars)
            && rule_avoid_mistaken_chars(&chars)
        {
            break;
        }
    }

    let password: String = chars.iter().collect();
    password
}

fn rule_avoid_mistaken_chars(sequence: &Vec<char>) -> bool {
    for c in sequence {
        if *c == 'i' || *c == 'o' || *c == 'l' {
            return false;
        }
    }
    true
}

fn rule_two_pairs(sequence: &Vec<char>) -> bool {
    let mut found_first = false;
    let mut first_last_idx = 0;
    for i in 0..sequence.len() - 1 {
        if found_first && i == first_last_idx {
            continue;
        }

        if sequence[i] != sequence[i + 1] {
            continue;
        }

        if !found_first {
            found_first = true;
            first_last_idx = i + 1;
            continue;
        }

        return true;
    }
    return false;
}

fn rule_increasing_straight(sequence: &Vec<char>) -> bool {
    for i in 0..sequence.len() - 2 {
        if sequence[i] as u8 + 1 == sequence[i + 1] as u8
            && sequence[i + 1] as u8 + 1 == sequence[i + 2] as u8
        {
            return true;
        }
    }

    return false;
}

fn increment(sequence: Vec<char>) -> Vec<char> {
    let mut idx = sequence.len() - 1;
    let mut out = sequence.clone();

    let mut new_char = sequence[idx];
    loop {
        new_char = char::from((((new_char as u8 + 1) - 97) % 26) + 97);
        if out[idx] == 'i' && out[idx] == 'o' && out[idx] == 'l' {
            continue;
        }
        out[idx] = new_char;

        if new_char == 'a' {
            idx -= 1;
            new_char = sequence[idx];
            continue;
        }

        break;
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_password() {
        // Arrange
        let input_expected = vec![("abcdefgh", "abcdffaa"), ("ghijklmn", "ghjaabcc")];

        for (input, expected) in input_expected {
            // Act
            let password = find_password(input);

            // Assert
            assert_eq!(password, expected);
        }
    }
}
