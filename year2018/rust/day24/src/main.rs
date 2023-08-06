use std::{fs, collections::{HashSet, BTreeSet}};
use anyhow::{Result, anyhow, Ok};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let mut grame = Game::from(&input)?;
    let part_1 = grame.play_game();

    println!("Part 1: {}", part_1);

    Ok(())
}

struct Game {
    immunes: Group,
    inflections: Group
}

impl Game {
    fn from(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let immunes = Group::from(parts[0])?;
        let inflections = Group::from(parts[1])?;

        Ok(Self{ immunes, inflections })
    }

    fn play_game(&mut self) -> u32{
        // let mut count = 0;
        loop {
            // count += 1;
            // println!("Rount: {}", count);
            // println!("Immunes:");
            // println!("{:?}", self.immunes);
            // println!("Inflections:");
            // println!("{:?}", self.inflections);

            self.play_round();

            if self.immunes.units.len() == 0 {
                return self.inflections.units
                    .iter()
                    .map(|u| u.units)
                    .sum::<u32>();
            }
            if self.inflections.units.len() == 0 {
                return self.immunes.units
                    .iter()
                    .map(|u| u.units)
                    .sum::<u32>();
            }
        }
    }
    fn play_round(&mut self) {
        // target selection
        let target_selections = self.find_target_selections();

        // attacking
        self.attack_selections(target_selections);

        // remove dead
        self.immunes.units.retain(|u| u.units > 0);
        self.inflections.units.retain(|u| u.units > 0);
    }

    fn attack_selections(&mut self, target_selections: Vec<(usize, usize)>) {
        let mut units= self.get_ordered_units();

        let mut attacking: Vec<(Unit, usize, usize)> = target_selections
            .iter()
            .map(|(attacker_id, defender_id)| (units[*attacker_id].clone(), *attacker_id, *defender_id))
            .collect();
        attacking.sort_by(|(a, _, _), (b, _, _)| b.initiative.cmp(&a.initiative));


        for (_, attacker_id, defender_id) in attacking {
            let attacker = units[attacker_id].clone();
            let defender = &mut units[defender_id];
            // if attacker.units == 0 || defender.units == 0{
            //     continue;
            // }
            let damage = defender.calculate_damage_taken(&attacker);
            defender.apply_damage(damage);
        }
    }

    fn get_ordered_units(&mut self) -> Vec<&mut Unit> {
        let mut selections: Vec<&mut Unit> = vec![];
        for immune in &mut self.immunes.units {
            selections.push(immune);
        }
        for inflection in &mut self.inflections.units {
            selections.push(inflection);
        }

        selections.sort_by(|a, b| {
            let a_effective = a.get_effective_power();
            let b_effective = b.get_effective_power();
            if a_effective == b_effective {
                b.initiative.cmp(&a.initiative)
            } else {
                b_effective.cmp(&a_effective)
            }
        });
        selections
    }

    fn find_target_selections(&mut self) -> Vec<(usize, usize)> {
        let selections = self.get_ordered_units();

        let mut defends: HashSet<usize> = HashSet::new();
        let mut target_selections = vec![];
        
        for (idx_target, select_target) in selections.iter().enumerate() {
            let mut targets = vec![];
            for (idx_defend, select_defend) in selections.iter().enumerate() {
                if defends.contains(&idx_defend) || select_target.group_type == select_defend.group_type {
                    continue;
                }
                targets.push((idx_defend, select_defend.calculate_damage_taken(&select_target)));
            }
            targets.sort_by(|(i_a, a), (i_b, b)| {
                if b == a {
                    let b_effective = &selections[*i_b].get_effective_power();
                    let a_effective = &selections[*i_a].get_effective_power();
                    if b_effective == a_effective {
                        *(&selections[*i_b].initiative.cmp(&selections[*i_a].initiative))
                    } else {
                        b_effective.cmp(&a_effective)
                    }
                } else {
                    b.cmp(&a)
                }
            });
            let Some((idx_defend, _)) = targets.iter().next() else { continue;};
            target_selections.push((idx_target, *idx_defend));
            defends.insert(*idx_defend);
        }
        target_selections
    }
}

#[derive(Debug, PartialEq, Clone)]
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
            let unit = Unit::from(line, group_type.clone())?;
            units.push(unit);
        };
        Ok(Self { units })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
struct Unit {
    units: u32,
    hit_points: u32,
    immunes: BTreeSet<Special>,
    weakness: BTreeSet<Special>,
    damage_power: u32,
    damage_type: Special,
    initiative: u32,
    group_type: GroupType,
}

impl Unit {
    fn apply_damage(&mut self, damage: u32) {
        let mut total_life = self.units * self.hit_points;
        total_life = total_life.saturating_sub(damage);
        let mut units = total_life / self.hit_points;
        if total_life % self.hit_points != 0 {
            units += 1;
        }
        self.units = units;
    }

    fn calculate_damage_taken(&self, attacker: &Unit) -> u32 {
        let mut damage = attacker.get_effective_power();
        if self.weakness.contains(&attacker.damage_type) {
            damage *= 2;
        }
        if self.immunes.contains(&attacker.damage_type) {
            damage = 0;
        }
        damage
    }

    fn get_effective_power(&self) -> u32 {
        self.units * self.damage_power
    }

    fn from(input: &str, group_type: GroupType) -> Result<Self> {
        let units_parts = input
            .split(" units each with ")
            .collect::<Vec<&str>>();
        let units: u32 = units_parts[0].parse()?;
        let hits_points_parts = units_parts[1]
            .split(" hit points ")
            .collect::<Vec<&str>>();
        let hit_points: u32 = hits_points_parts[0].parse()?;
        let mut immunes : BTreeSet<Special> = BTreeSet::new();
        let mut weakness : BTreeSet<Special> = BTreeSet::new();
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
                        SpecialType::Weak => weakness.insert(special_matched),
                        SpecialType::Immune => immunes.insert(special_matched),
                    };
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
            initiative,
            group_type
        })



    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // #[ignore = "not ready"]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day24_test_data.txt")?;

        // Act
        let mut game = Game::from(&input)?;
        let actual = game.play_game();

        // Assert
        assert_eq!(actual, 5_216);
        Ok(())
    }

    #[test]
    fn test_attack_selection() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day24_test_data.txt")?;
        let mut game = Game::from(&input)?;
        let target_selections = game.find_target_selections();

        // Act
        game.attack_selections(target_selections);
        
        // Assert
        assert_eq!(game.immunes.units[0].units, 0);
        assert_eq!(game.immunes.units[1].units, 905);
        assert_eq!(game.inflections.units[0].units, 797);
        assert_eq!(game.inflections.units[1].units, 4_434);
        Ok(())
    }

    #[test]
    fn test_find_target_selections() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day24_test_data.txt")?;
        let mut game = Game::from(&input)?;
    
        // Act
        let target_selections = game.find_target_selections();
        let units = game.get_ordered_units();

        // Assert
        assert_eq!(target_selections.len(), 4);
        // Infection 1 attracks
        assert_eq!(units[target_selections[0].0].units, 801);
        // Immune 1 defend
        assert_eq!(units[target_selections[0].1].units, 17);
        // Immune 1 attracks
        assert_eq!(units[target_selections[1].0].units, 17);
        // Infection 2 defend
        assert_eq!(units[target_selections[1].1].units, 4_485);  
        // Infection 2 attracks
        assert_eq!(units[target_selections[2].0].units, 4_485);
        // Immune 2 defend
        assert_eq!(units[target_selections[2].1].units, 989);              
        // Immune 2 attracks
        assert_eq!(units[target_selections[3].0].units, 989);
        // Infection 1 defend
        assert_eq!(units[target_selections[3].1].units, 801);    
        Ok(())
    }

    #[test]
    fn test_get_ordered_units() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day24_test_data.txt")?;
        let mut game = Game::from(&input)?;

        // Act
        let units = game.get_ordered_units();

        // Assert
        assert_eq!(units.len(), 4);
        assert_eq!(units[0].group_type, GroupType::Inflection);
        assert_eq!(units[0].units, 801);
        assert_eq!(units[1].group_type, GroupType::Immune);
        assert_eq!(units[1].units, 17);        
        assert_eq!(units[2].group_type, GroupType::Inflection);
        assert_eq!(units[2].units, 4_485);        
        assert_eq!(units[3].group_type, GroupType::Immune);
        assert_eq!(units[3].units, 989);
        Ok(())
    }

    #[test]
    fn test_unit_from() -> Result<()> {
        // Arrange
        let input = "1155 units each with 5643 hit points (weak to bludgeoning; immune to cold) with an attack that does 42 slashing damage at initiative 15";
        let expected = Unit {
            units: 1155,
            hit_points: 5643,
            weakness: BTreeSet::from([Special::Bludgeoning]),
            immunes: BTreeSet::from([Special::Cold]),
            damage_power: 42,
            damage_type: Special::Slashing,
            initiative: 15,
            group_type: GroupType::Immune
        };

        // Act
        let actual = Unit::from(input, GroupType::Immune)?;

        // Assert
        assert_eq!(actual, expected);
        Ok(())
    }
}