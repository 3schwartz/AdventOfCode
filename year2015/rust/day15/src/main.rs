use std::fs;

use anyhow::Result;

fn part_1(ingrediens: &Vec<Vec<i32>>) -> i32 {
    let mut max = i32::MIN;
    for sugar in 0..=100 {
        for sprinkles in 0..=100 {
            if sugar + sprinkles > 100 {
                break;
            }
            for candy in 0..=100 {
                if sugar + sprinkles + candy > 100 {
                    break;
                }
                for chocolate in 0..=100 {
                    if sugar + sprinkles + candy + chocolate > 100 {
                        break;
                    }
                    if sugar + sprinkles + candy + chocolate < 100 {
                        continue;
                    }
                    let capacity = ingrediens[0][0] * sugar
                    + ingrediens[1][0] * sprinkles
                    + ingrediens[2][0] * candy
                    + ingrediens[3][0] * chocolate;

                    let durability = ingrediens[0][1] * sugar
                    + ingrediens[1][1] * sprinkles
                    + ingrediens[2][1] * candy
                    + ingrediens[3][1] * chocolate;

                    let flavor = ingrediens[0][2] * sugar
                    + ingrediens[1][2] * sprinkles
                    + ingrediens[2][2] * candy
                    + ingrediens[3][2] * chocolate;

                    let texture = ingrediens[0][3] * sugar
                    + ingrediens[1][3] * sprinkles
                    + ingrediens[2][3] * candy
                    + ingrediens[3][3] * chocolate;

                    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
                        continue;
                    }

                    let total = capacity * durability * flavor * texture;
                    if total > max {
                        max = total;
                    }
                }
            }
        }
    }
    return max;
}

fn part_2(ingrediens: &Vec<Vec<i32>>) -> i32 {
    let mut max = i32::MIN;
    for sugar in 0..=100 {
        for sprinkles in 0..=100 {
            if sugar + sprinkles > 100 {
                break;
            }
            if ingrediens[0][4] * sugar + ingrediens[1][4] * sprinkles > 500 {
                break;
            }
            for candy in 0..=100 {
                if sugar + sprinkles + candy > 100 {
                    break;
                }
                if ingrediens[0][4] * sugar + ingrediens[1][4] * sprinkles + ingrediens[2][4] * candy > 500 {
                    break;
                }
                for chocolate in 0..=100 {
                    if sugar + sprinkles + candy + chocolate > 100 {
                        break;
                    }
                    if sugar + sprinkles + candy + chocolate < 100 {
                        continue;
                    }
                    if ingrediens[0][4] * sugar + ingrediens[1][4] * sprinkles + ingrediens[2][4] * candy + ingrediens[3][4] * chocolate > 500 {
                        break;
                    }
                    if ingrediens[0][4] * sugar + ingrediens[1][4] * sprinkles + ingrediens[2][4] * candy + ingrediens[3][4] * chocolate  < 500 {
                        continue;
                    }
                    let capacity = ingrediens[0][0] * sugar
                    + ingrediens[1][0] * sprinkles
                    + ingrediens[2][0] * candy
                    + ingrediens[3][0] * chocolate;

                    let durability = ingrediens[0][1] * sugar
                    + ingrediens[1][1] * sprinkles
                    + ingrediens[2][1] * candy
                    + ingrediens[3][1] * chocolate;

                    let flavor = ingrediens[0][2] * sugar
                    + ingrediens[1][2] * sprinkles
                    + ingrediens[2][2] * candy
                    + ingrediens[3][2] * chocolate;

                    let texture = ingrediens[0][3] * sugar
                    + ingrediens[1][3] * sprinkles
                    + ingrediens[2][3] * candy
                    + ingrediens[3][3] * chocolate;

                    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
                        continue;
                    }

                    let total = capacity * durability * flavor * texture;
                    if total > max {
                        max = total;
                    }
                }
            }
        }
    }
    return max;
}

fn main() -> Result<()>{
    let input = fs::read_to_string("../data/day15_data.txt")?;

    let mut ingrediens = vec![];

    for line in input.lines() {
        let numbers: Vec<i32> = line.split(' ')
        .map(|s| s.trim_end_matches(','))
            .flat_map(|s| s.parse::<i32>())
            .collect();

        ingrediens.push(numbers);
    }

    let part_1 = part_1(&ingrediens);
    
    println!("Part 1: {}",part_1);

    let part_2 = part_2(&ingrediens);

    println!("Part 2: {}",part_2);

    Ok(())
}
