use std::{fs, rc::Rc, cell::{RefCell, Ref}, collections::HashMap, str::Chars};

fn main() {
    let input = fs::read_to_string("../../data/day5_data.txt")
        .expect("couldn't open file");
    let length = get_polymer_length(input.trim());
    println!("Part 1: {}", length)
}

fn get_polymer_length(polymer: &str) -> usize {
    let mut chars = polymer.chars();

    let polymer = Polymer::new(chars);

    polymer.react();

    let length = polymer.length();
    length
}

#[test]
fn test_get_polymer_length() {
    let input = "dabAcCaCBAcCcaDA";
    let length = get_polymer_length(input);
    assert_eq!(length, 10);
}

struct Unit {
    character: char,
    id: u32,
    next: u32,
}
struct Polymer{
    polymer: HashMap<u32, Unit>,
    start: u32
}

impl Polymer{
    fn new (chars : Chars) -> Self {
        let polymer = HashMap::new();
        let idx: u32 = 0;
        for c in chars {
            polymer.insert(idx, Unit::new(c, idx, idx + 1));
            idx+=1;
        }
        Self {
            polymer,
            start: 0
        }
    }

    fn react(self) {
        let mut current_length = self.polymer.len();

        loop {
            let new_length = self.polymer.len();



            if new_length == current_length {
                break;
            }
            current_length = new_length;
        }
    }

    fn get_start(&mut self) -> Option<&Unit> {
        self.polymer.get(&self.start)
    }

    fn get(&mut self, idx: &u32) -> Option<&Unit> {
        self.polymer.get(idx)
    }

    fn remove(&mut self, idx: &u32) {
        self.polymer.remove(idx);
    }

    fn react_loop(&mut self) {
        // let polymer = &mut self.polymer;
        let current = self.get_start();
        let mut last : Option<&Unit> = None;

        loop {
            match current {
                Some(this) => {
                    let match_next = self.get(&this.next)
                        .map_or(false, |f| self.characters_match(this, f));
                    
                    if match_next {
                        self.remove(&this.id);
                        self.remove(&this.next);
                        match last {
                            Some(last_unit) => {
                                last_unit.next = self.polymer.get(&this.next)
                                    .map_or(last_unit.next + 1, |f| f.next);
                            },
                            None => {
                                self.start = self.polymer.get(&this.next)
                                    .map_or(self.start + 1, |f| f.id)
                            },
                        }
                        break;
                    }

                    last = current;
                    current = self.polymer.get(&this.next)
                },
                None => break,
            }
        }
        
    }

    fn characters_match(&mut self, current: &Unit, next: &Unit) -> bool {
        (current.character.is_ascii_lowercase() && next.character.is_ascii_uppercase() ||
        current.character.is_ascii_uppercase() && next.character.is_ascii_lowercase()) &&
        current.character.to_ascii_lowercase() == next.character.to_ascii_lowercase()
    }

    fn length(self) -> usize {
        self.polymer.len()
    }
}

impl Unit {
    fn new (character: char, id: u32, next: u32) -> Self {
        Self {
            character, id, next
        }
    }
}
