use std::{fs, collections::HashSet, cmp};

use anyhow::{Result, anyhow, Ok};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Item {
    name: String,
    cost: u32,
    damage: u32,
    armor: u32
}

impl Item {
    fn from(parts: Vec<&str>) -> Result<Item> {
        if parts.len() == 4 {
            return Ok(Item {
                name: parts[0].to_string(),
                cost: parts[1].parse()?,
                damage: parts[2].parse()?,
                armor: parts[3].parse()?,
            })
        }
        if parts.len() == 5 {
            return Ok(Item {
                name: parts[0].to_owned() + parts[1],
                cost: parts[2].parse()?,
                damage: parts[3].parse()?,
                armor: parts[4].parse()?,
            })
        }
        Err(anyhow!("not able to create item from: {:?}", parts))
    }
}

#[derive(Debug)]
struct Shop {
    weapons: HashSet<Item>,
    armor: HashSet<Item>,
    rings: HashSet<Item>
}

impl Shop {
    fn from(info: &str) -> Result<Shop> {
    
        let parts: Vec<&str> = info.split("\n\n").collect();
        if parts.len() != 3 {
            return Err(anyhow!("parts length not correct: {:?}", parts));
        }

        let weapons = Shop::create_set(parts[0])?;
        let armors = Shop::create_set(parts[1])?;
        let rings = Shop::create_set(parts[2])?;
    
        Ok(Shop{weapons, armor: armors, rings })
    }

    fn create_set(part: &str) -> Result<HashSet<Item>> {
        let mut items = HashSet::new();

        for line in part.lines().skip(1) {
            let infos: Vec<&str> = line.split_whitespace().collect();
            let item = Item::from(infos)?;
            items.insert(item);
        }
        Ok(items)
    }
}


#[derive(Eq, PartialEq, Hash, Clone)]
struct Player {
    hit_point: u32,
    damage_init: u32,
    armor_init: u32,
    weapon: Option<Item>,
    armor: Option<Item>,
    rings: (Option<Item>, Option<Item>)
}

impl Player {
    fn new(hit_point: u32, damage_init: u32, armor_init: u32) -> Self {
        Self { hit_point, damage_init, armor_init,
             weapon: None, armor: None, rings: (None,None) }
    }

    fn from(input: &str) -> Result<Player> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.len() != 3 {
            return Err(anyhow!("enemy input to able to be parsed to player: {:?}", input));
        }
        let hit_point: u32 = lines[0].split_whitespace().last()
            .ok_or_else(|| anyhow!("not able to parse player: {:?}", lines))?
            .parse()?;
        let damage_init: u32 = lines[1].split_whitespace().last()
            .ok_or_else(|| anyhow!("not able to parse player: {:?}", lines))?
            .parse()?;
        let armor_init: u32 = lines[2].split_whitespace().last()
            .ok_or_else(|| anyhow!("not able to parse player: {:?}", lines))?
            .parse()?;

        Ok(Player::new(hit_point, damage_init, armor_init))
    }

    fn beats(&self, enemy: &Player) -> bool {
        let own_damage = self.get_damage();
        let own_armor = self.get_armor();
        let enemy_damage = enemy.get_damage();
        let enemy_armor = enemy.get_armor();

        let own_hit = cmp::max(own_damage.saturating_sub(enemy_armor), 1);
        let enemy_hit = cmp::max(enemy_damage.saturating_sub(own_armor), 1);

        return own_hit >= enemy_hit;
    }

    fn get_damage(&self) -> u32 {
        return self.damage_init + 
                self.weapon.as_ref()
                .map_or(0, |i| i.damage) + 
                self.rings.0.as_ref()
                .map_or(0, |i| i.damage) + 
                self.rings.1.as_ref()
                .map_or(0, |i| i.damage);
    }

    fn get_armor(&self) -> u32 {
        return self.armor_init + 
                self.armor.as_ref()
                .map_or(0, |i| i.armor) + 
                self.rings.0.as_ref()
                .map_or(0, |i| i.armor) + 
                self.rings.1.as_ref()
                .map_or(0, |i| i.armor);
    }

    fn get_cost(&self) -> u32 {
        return
                self.weapon.as_ref()
                .map_or(0, |i| i.cost) + 
                self.armor.as_ref()
                .map_or(0, |i| i.cost) + 
                self.rings.0.as_ref()
                .map_or(0, |i| i.cost) + 
                self.rings.1.as_ref()
                .map_or(0, |i| i.cost);
    }

    fn specification_cost(&self, enemy: &Player, shop: &Shop, win: bool, least_amount: bool) -> Result<u32> {
        let start = self.clone();
        
        let mut visited: HashSet<Player> = HashSet::new();
        let mut queue: Vec<Player> = Vec::from([start]);
        let mut boundary_cost = if least_amount { u32::MAX } else { u32::MIN};
        while let Some(player) = queue.pop() {
            if !visited.insert(player.clone()) {
                continue;
            }
            let cost =  player.get_cost();
            if least_amount && cost >= boundary_cost {
                continue;
            }
            if player.beats(&enemy) == win && player.weapon.is_some() &&
                 (least_amount && cost < boundary_cost || !least_amount && cost > boundary_cost) {
                boundary_cost = cost;
            }
            if player.weapon.is_none() {
                for weapon in &shop.weapons {
                    let mut clone = player.clone();
                    clone.weapon = Some(weapon.clone());
                    queue.push(clone);
                }
            }
            if player.armor.is_none() {
                for armor in &shop.armor {
                    let mut clone = player.clone();
                    clone.armor = Some(armor.clone());
                    queue.push(clone);
                }
            }
            if player.rings.0.is_none() {
                for ring in &shop.rings {
                    let mut clone = player.clone();
                    clone.rings.0 = Some(ring.clone());
                    queue.push(clone);
                }
            }
            if player.rings.1.is_none() && player.rings.0.is_some() {
                for ring in &shop.rings {
                    if *ring == *player.rings.0.as_ref().ok_or_else(|| anyhow!("ring 0 should be there"))? {
                        continue;
                    }
                    let mut clone = player.clone();
                    clone.rings.1 = Some(ring.clone());
                    queue.push(clone);
                }
            }
        }
        Ok(boundary_cost)
    }
}


fn main() -> Result<()> {
    let info = fs::read_to_string("../data/day21_info.txt")?;
    let data = fs::read_to_string("../data/day21_data.txt")?;

    let shop = Shop::from(&info)?;

    let enemy = Player::from(&data)?;
    let start = Player::new(100, 0, 0);

    let min_cost = start.specification_cost(&enemy, &shop, true, true)?;

    println!("Part 1: {}", min_cost);

    let max_cost_loose = start.specification_cost(&enemy, &shop, false, false)?;

    println!("Part 2: {}", max_cost_loose);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_items_equals() {
        // Arrange
        let item_one = Item{armor: 0, name: "foo".to_owned(), cost: 0, damage: 0};
        let item_second = Item{armor: 0, name: "foo".to_owned(), cost: 0, damage: 0};

        // Act
        let equals = item_one == item_second;

        // Assert
        assert!(equals);
    }
}