use anyhow::Result;
use regex::Regex;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day8_data.txt")?;

    let mut string_lenght = 0;
    let mut memory_length = 0;
    let mut encoded_length = 0;

    for line in input.lines() {
        let (string_line, memory_line) = get_decoded_lengths(&line)?;

        string_lenght += string_line;
        memory_length += memory_line;

        let (_, encoded_line_length) = get_encoded_lengths(&line)?;
        encoded_length += encoded_line_length;
    }

    let part_1 = string_lenght - memory_length;

    println!("Part 1: {part_1}");

    let part_2 = encoded_length - string_lenght;

    println!("Part 2: {part_2}");

    Ok(())
}

fn get_decoded_lengths(line: &str) -> Result<(usize, usize)> {
    let string_lenght = get_string_literals_length(&line);
    
    let hex_pattern = Regex::new(r"\\x([0-9a-fA-F]{2})")?;
    let matches = hex_pattern.captures_iter(line);
    let quote_matches = line.matches(r#"\""#).count();
    let escape_matches = line.matches(r"\\").count();

    let mut line_memory_lenght = line.len() - 2;
    line_memory_lenght -= matches.count() * 3;
    line_memory_lenght -= quote_matches;
    line_memory_lenght -= escape_matches;

    return Ok((string_lenght, line_memory_lenght));
}

fn get_string_literals_length(line: &str) -> usize {
    let white_space = get_white_space_count(&line);

    let chars_length = line.chars().count();

    let string_lenght = chars_length - white_space;
    string_lenght
}

fn get_white_space_count(line: &str) -> usize {
    let white_space = line.matches(r"\s").count()
        + line.matches(r"\r").count()
        + line.matches(r"\t").count()
        + line.matches(r"\n").count();
    return white_space
}

fn get_encoded_lengths(line: &str) -> Result<(usize, usize)> {
    let string_lenght = get_string_literals_length(&line);
    
    let white_space = get_white_space_count(&line);

    let escape_matches = line.matches(r"\").count();
    let quote_matches = line.matches(r#"""#).count();

    let mut line_escaped_length = line.len() + 2;
    line_escaped_length += escape_matches;
    line_escaped_length += quote_matches;
    line_escaped_length -= white_space;

    return Ok((string_lenght, line_escaped_length));
}

#[cfg(test)]
mod test {
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn test_print_correct_ascii_extended() -> Result<()>{
        // Arrange
        let input = "fb";

        // Act
        let byte = u8::from_str_radix(input, 16)?;

        let ascii_extended = char::from_u32(byte as u32)
            .ok_or(anyhow!("unable to map: {}", byte))?
            .to_string();

        println!("{}", ascii_extended);
        assert_eq!("û", ascii_extended);
        Ok(())
    }

    #[test]
    fn test_get_white_space() {
        // Arrange
        let input = r"foo\tbar";

        // Act
        let count = get_white_space_count(&input);

        // Assert
        assert_eq!(count, 1);
    }

    #[test]
    fn test_correct_encoded_lengths() {
        // Arrange
        let inputs = vec![
            (r#""""#, (2, 6)),
            (r#""abc""#, (5, 9)),
            (r#""aaa\"aaa""#, (10, 16)),
            (r#""\x27""#, (6, 11)),
        ];

        // Act
        for (input, expected) in inputs {
            let lengths = get_encoded_lengths(&input).unwrap();

            // Assert
            assert_eq!(lengths, expected);
        }
    }

    #[test]
    fn test_correct_decoded_lengths() {
        // Arrange
        let inputs = vec![
            (r#""""#, (2, 0)),
            (r#""abc""#, (5, 3)),
            (r#""aaa\"aaa""#, (10, 7)),
            (r#""\x27""#, (6, 1)),
        ];

        // Act
        for (input, expected) in inputs {
            let lengths = get_decoded_lengths(&input).unwrap();

            // Assert
            assert_eq!(lengths, expected);
        }
    }

}
