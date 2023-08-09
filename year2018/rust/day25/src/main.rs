use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day25_data.txt")?;

    let constellations = find_constellations(&input);

    println!("Part 1: {}", constellations);
    Ok(())
}

fn find_constellations(input: &str) -> usize {
    let mut set = vec![];
    for line in input.lines() {
        let coord = line.split(',')
            .map(|c| c.trim().parse().unwrap())
            .collect::<Vec<i32>>();
        set.push(vec![coord]);
    }

    loop {
        let mut found : Option<(usize, usize)> = None;
        for (f_idx, first) in set.iter().enumerate() {
            for first_vec in first {
                for (s_idx, second) in set.iter().enumerate() {
                    for second_vec in second {
                        if f_idx == s_idx {
                            continue;
                        }
                        let distance = first_vec.iter().zip(second_vec)
                            .map(|(f, s)| (f-s).abs())
                            .sum::<i32>();
                        if distance <= 3 {
                            found = Some((f_idx, s_idx));
                            break;
                        }
                    }
                    if found.is_some() {
                        break;
                    }
                }
                if found.is_some() {
                    break;
                }
            }
            if found.is_some() {
                break;
            }
        }
        let Some((f_idx, s_idx)) = found else { break; };
        let mut to_append = set.remove(s_idx);
        set[f_idx].append(&mut to_append);
    }
    set.len()
}

#[cfg(test)]
mod test {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day25_test1_data.txt")?;

        // Act
        let actual = find_constellations(&input);

        // Assert
        assert_eq!(actual, 4);
        Ok(())
    }
}
