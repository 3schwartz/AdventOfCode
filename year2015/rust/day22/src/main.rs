use std::{fs, collections::{BTreeMap, BTreeSet}, cmp};

use anyhow::{Result,anyhow};

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Player {
    hit_point: i32,
    damage_init: i32,
    armor_init: i32,
    mana: i32,
    damage: i32,
    armor: i32
}

impl Player {
    fn new(hit_point: i32, mana: i32) -> Self {
        Self { hit_point, damage_init: 0, armor_init: 0, mana,
             damage: 0, armor: 0
            }
    }

    fn from(input: &str) -> Result<Player> {
        let lines : Vec<&str> = input.lines().collect();
        if lines.len() != 2 {
            return Err(anyhow!("not able to create player: {:?}", input))
        }
        let hit_point: i32 = lines[0].split_ascii_whitespace().last()
            .ok_or_else(|| anyhow!("not able to parse player: {:?}", lines[0]))?
            .parse()?;
        let damage_init: i32 = lines[1].split_ascii_whitespace().last()
            .ok_or_else(|| anyhow!("not able to parse player: {:?}", lines[1]))?
            .parse()?;

        Ok(Player{ hit_point, damage_init, armor_init: 0, mana: 0,
            damage: 0, armor: 0
        })
    }

    fn reset(&mut self) {
        self.damage = 0;
        self.armor = 0;
    }

    fn is_dead(&self) -> bool {
        return self.hit_point <= 0;
    }

    fn deal_damage(&self, other: &mut Player) {
        let total_damage = cmp::max(self.get_damage() - other.get_armor(), 1);
        other.hit_point -= total_damage;
    }

    fn get_damage(&self) -> i32 {
        return self.damage + self.damage_init;
    }

    fn get_armor(&self) -> i32 {
        return self.armor + self.armor_init;
    }

    fn can_affort(&self, spell: &Spell) -> bool {
        return spell.get_cost() <= self.mana;
    }

    fn win_with_least_amount_of_mana(&self, enemy: &Player) -> i32 {
        let mut visisted: BTreeSet<State> = BTreeSet::new();

        let mut queue: Vec<State> = Vec::new();
        let mut boundary = i32::MAX;

        for spell in Spell::iter() {
            let mut state = State::new(self.clone(), enemy.clone());
            state.apply_spell(spell);
            queue.push(state);
        }

        while let Some(mut state) = queue.pop() {
            if state.total_mana >= boundary || visisted.contains(&state) {
                continue;
            }
            visisted.insert(state.clone());
            
            // Boss turn ///
            state.apply_effects();
            if state.enemy.is_dead() {
                boundary = state.total_mana;
                continue;
            }

            state.enemy.deal_damage(&mut state.player);
            if state.player.is_dead() {
                continue;
            }

            state.reset();

            // Players turn ///
            state.apply_effects();
            if state.enemy.is_dead() {
                boundary = state.total_mana;
                continue;
            }

            for spell in Spell::iter() {
                if state.spells.contains_key(&spell) {
                    continue;
                }
                if !state.player.can_affort(&spell) {
                    continue;
                }
                
                let mut state_cloned = state.clone();
                state_cloned.apply_spell(spell);

                if state_cloned.total_mana >= boundary {
                    continue;
                }
                if state_cloned.enemy.is_dead() {
                    boundary = state_cloned.total_mana;
                    continue;
                }
                
                state_cloned.reset();
                queue.push(state_cloned);
            }
        }

        boundary
    }
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
struct State {
    spells: BTreeMap<Spell, u32>,
    player: Player,
    enemy: Player,
    total_mana: i32
}

impl State {
    fn new(player: Player, enemy: Player) -> Self {
        Self { spells: BTreeMap::new(), player, enemy, total_mana: 0 }
    }

    fn reset(&mut self) {
        self.player.reset();
        self.enemy.reset();
        self.spells.retain(|_,v| *v > 0);
    }

    fn apply_effects(&mut self) {
        for (spell, count) in &mut self.spells {
            spell.apply_effect(&mut self.player, &mut self.enemy);
            *count = count.saturating_sub(1);
        }
    }

    fn apply_spell(&mut self, spell: Spell) {
        let (turns, cost) = spell.turns_and_cost();
        spell.apply_cost(&mut self.player);
        spell.apply_damage(&mut self.player, &mut self.enemy);
        self.spells.insert(spell, turns);
        self.total_mana += cost;
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

struct SpellIterator {
    next: u32
}

impl Iterator for SpellIterator {
    type Item = Spell;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.next {
            0 => Some(Spell::MagicMissile),
            1 => Some(Spell::Drain),
            2 => Some(Spell::Shield),
            3 => Some(Spell::Poison),
            4 => Some(Spell::Recharge),
            _ => None
        };
        self.next += 1;
        result
    }
}


impl Spell {
    const MAGIC_MISSILE_COST: i32 = 53;
    const DRAIN_COST: i32 = 73;
    const SHIELD_COST: i32 = 113;
    const POISON_COST: i32 = 173;
    const RECHARGE_COST: i32 = 229;
    
    fn iter() -> SpellIterator {
        SpellIterator { next: 0 }
    }

    fn get_cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => Spell::MAGIC_MISSILE_COST,
            Spell::Drain => Spell::DRAIN_COST,
            Spell::Shield => Spell::SHIELD_COST,
            Spell::Poison => Spell::POISON_COST,
            Spell::Recharge => Spell::RECHARGE_COST,
        }
    }

    fn turns_and_cost(&self) -> (u32, i32) {
        match self {
            Spell::MagicMissile => (0, Spell::MAGIC_MISSILE_COST),
            Spell::Drain => (0, Spell::DRAIN_COST),
            Spell::Shield => (6, Spell::SHIELD_COST),
            Spell::Poison => (6, Spell::POISON_COST),
            Spell::Recharge => (5, Spell::RECHARGE_COST),
        }
    }

    fn apply_cost(&self, player: &mut Player) {
        player.mana -= self.get_cost();
    }

    fn apply_damage(&self, player: &mut Player, enemy: &mut Player) {
        match self {
            Spell::MagicMissile => enemy.hit_point -= 4,
            Spell::Drain => {
                player.hit_point += 2;
                enemy.hit_point -= 2;
            },
            Spell::Shield | Spell::Poison | Spell::Recharge => (),
        }
    }

    fn apply_effect(&self, player: &mut Player, enemy: &mut Player) {
        match self {
            Spell::MagicMissile | Spell::Drain => (),
            Spell::Shield => player.armor += 7,
            Spell::Poison => enemy.hit_point -= 3,
            Spell::Recharge => player.mana += 101,
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day22_data.txt")?;
    let player = Player::new(50, 500);
    let enemy = Player::from(&input)?;

    let part_1 = player.win_with_least_amount_of_mana(&enemy);

    println!("Part 1: {}", part_1);
    Ok(())
}
