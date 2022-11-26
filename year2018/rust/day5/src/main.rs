use std::{fs, collections::HashMap, str::Chars};

fn main() {
    let input = fs::read_to_string("../../data/day5_data.txt")
        .expect("couldn't open file");
    let length = get_polymer_length(input.trim());
    println!("Part 1: {}", length)
}

fn get_polymer_length(polymer: &str) -> usize {
    let chars = polymer.chars();

    let mut polymer = Polymer::new(chars);

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
    polymer: HashMap<u32,Box<Unit>>,
    start: u32
}

impl Polymer{
    fn new (chars : Chars) -> Polymer {
        let mut polymer: HashMap<u32, Box<Unit>> = HashMap::new();
        let mut idx: u32 = 0;
        for c in chars {
            polymer.insert(idx, Unit::new(c, idx, idx + 1));
            idx+=1;
        }
        Self {
            polymer,
            start: 0
        }
    }

    fn react(&mut self) {
        let mut current_length = self.polymer.len();

        loop {
            let new_length = self.polymer.len();

            self.react_loop();

            if new_length == current_length {
                break;
            }
            current_length = new_length;
        }
    }

    fn get(&self, idx: &u32) -> Option<&Box<Unit>> {
        self.polymer.get(idx)
    }

    fn remove(&mut self, idx: &u32) {
        self.polymer.remove(idx);
    }

    fn react_loop(&mut self) {
        let mut current = self.get(&self.start);
        let mut last : Option<&Box<Unit>> = None;

        loop {
            match current {
                Some(this) => {
                    let match_next = self.get(&this.next)
                        .map_or(false, |f| this.characters_match(f));
                    
                    if match_next {
                        // self.polymer.remove(&this.id);
                        // self.polymer.remove(&this.next);
                        match last {
                            Some(last_unit) => {
                                let next = self.polymer.get(&this.next)
                                    .map_or(last_unit.next + 1, |f| f.next);
                                self.polymer.insert(last_unit.id, Unit::new(last_unit.character, last_unit.id, next));
                            },
                            None => {
                                self.start = self.polymer.get(&this.next)
                                    .map_or(self.start + 1, |f| f.id)
                            },
                        }
                        break;
                    }

                    last = current;
                    current = self.get(&this.next)
                },
                None => break,
            }
        }
        
    }

    fn length(&self) -> usize {
        // self.polymer.len();
        let mut idx = self.start;
        let mut sum = 0;
        loop {
            match self.polymer.get(&idx) {
                Some(temp) => {
                    sum += 1;
                    idx = temp.next;
                },
                None => break,
            }
        };
        return sum;
    }
}

impl Unit {
    fn new (character: char, id: u32, next: u32) -> Box<Self> {
        Box::new(Self {
            character, id, next
        })
    }

    fn characters_match(self: &Box<Self>, next: &Unit) -> bool {
        (self.character.is_ascii_lowercase() && next.character.is_ascii_uppercase() ||
         self.character.is_ascii_uppercase() && next.character.is_ascii_lowercase()) &&
         self.character.to_ascii_lowercase() == next.character.to_ascii_lowercase()
    }
}