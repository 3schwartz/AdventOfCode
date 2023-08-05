use std::fs;
use anyhow::{Result, anyhow, Ok};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let parts: Vec<&str> = input.split("\n\n").collect();
    let immune = Group::from(parts[0])?;
    let inflection = Group::from(parts[1]);

    println!("{:?}", immune);
    println!("{:?}", inflection);

    Ok(())
}

#[derive(Debug)]
enum GroupType {
    Immune,
    Inflection,
}

impl GroupType {
    fn new(input: &str) -> Self {
        match input.contains("Immune System") {
            true => GroupType::Immune,
            false => GroupType::Inflection,
        }
    }
}

#[derive(Debug)]
struct Group {
    group_type: GroupType,
    units: Vec<Unit>,
}

impl Group {
    fn from(input: &str) -> Result<Self> {
        let mut lines = input.lines();
        let first = lines
            .next()
            .ok_or_else(|| anyhow!("first line should exist when creating group: {:?}", input))?;
        let group_type = GroupType::new(first);
        let mut units = vec![];
        for line in lines {
            let unit = Unit::from(line)?;
            units.push(unit);
        };
        Ok(Self { group_type, units })
    }
}

#[derive(Debug, PartialEq)]
enum Special {
    Cold,
    Bludgeoning,
    Slashing,
    Fire,
    Radiation,
}

impl Special {
    fn from(input: &str) -> Result<Self> {
        match input {
            "cold" => Ok(Special::Cold),
            "bludgeoning" => Ok(Special::Bludgeoning),
            "slashing" => Ok(Special::Slashing),
            "fire" => Ok(Special::Fire),
            "radiation" => Ok(Special::Radiation),
            _ => Err(anyhow!("special error: {}", input))
        }
    }
}

enum SpecialType {
    Weak,
    Immune
}

impl SpecialType {
    fn from(input: &str) -> Result<Self> {
        match input {
            "weak" => Ok(SpecialType::Weak),
            "immune" => Ok(SpecialType::Immune),
            _ => Err(anyhow!("special type error: {}", input))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Unit {
    units: u32,
    hit_points: u32,
    immunes: Vec<Special>,
    weakness: Vec<Special>,
    damage_power: u32,
    damage_type: Special,
    initiative: u32,
}

impl Unit {
    fn from(input: &str) -> Result<Self> {
        let units_parts = input
            .split(" units each with ")
            .collect::<Vec<&str>>();
        let units: u32 = units_parts[0].parse()?;
        let hits_points_parts = units_parts[1]
            .split(" hit points ")
            .collect::<Vec<&str>>();
        let hit_points: u32 = hits_points_parts[0].parse()?;
        let mut immunes : Vec<Special> = vec![];
        let mut weakness : Vec<Special> = vec![];
        let second_part = if hits_points_parts[1].starts_with('(') {
            let specials = hits_points_parts[1]
                .trim_start_matches('(')
                .split(')')
                .collect::<Vec<&str>>();
            let special_parts = specials[0]
                .split("; ")
                .collect::<Vec<&str>>();
            for special_part in special_parts {
                let special_line = special_part
                    .split(" to ")
                    .collect::<Vec<&str>>();
                let special_type = SpecialType::from(special_line[0])?;
                let specials_to_type = special_line[1]
                    .split(", ")
                    .collect::<Vec<&str>>();
                for special in specials_to_type {
                    let special_matched = Special::from(special)?;
                    match special_type {
                        SpecialType::Weak => weakness.push(special_matched),
                        SpecialType::Immune => immunes.push(special_matched),
                    }
                }
            }
            specials[1]
        } else {
            hits_points_parts[1]
        };
        let final_part = second_part
            .trim()
            .trim_start_matches("with an attack that does ")
            .split(" ")
            .collect::<Vec<&str>>();
        let damage_power: u32 = final_part[0].parse()?;
        let damage_type = Special::from(final_part[1])?;
        let initiative: u32 = final_part[5].parse()?;


        Ok(Self { 
            units,
            hit_points,
            immunes,
            weakness,
            damage_power,
            damage_type,
            initiative
        })



    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unit_from() -> Result<()> {
        // Arrange
        let input = "1155 units each with 5643 hit points (weak to bludgeoning; immune to cold) with an attack that does 42 slashing damage at initiative 15";
        let expected = Unit {
            units: 1155,
            hit_points: 5643,
            weakness: vec![Special::Bludgeoning],
            immunes: vec![Special::Cold],
            damage_power: 42,
            damage_type: Special::Slashing,
            initiative: 15,
        };

        // Act
        let actual = Unit::from(input)?;

        // Assert
        assert_eq!(actual, expected);
        Ok(())
    }
}