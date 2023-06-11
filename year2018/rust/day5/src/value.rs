use std::{collections::HashMap, str::Chars};

pub fn get_polymer_length(polymer: &str) -> usize {
    let chars = polymer.chars();

    let mut polymer = Polymer::new(chars);

    polymer.react();

    let length = polymer.length();
    length
}

#[derive(Copy, Clone)]
struct Unit {
    character: char,
    id: u32,
    next: u32,
}
struct Polymer {
    polymer: HashMap<u32, Unit>,
    start: u32,
}

impl Polymer {
    fn new(chars: Chars) -> Polymer {
        let mut polymer: HashMap<u32, Unit> = HashMap::new();
        let mut idx: u32 = 0;
        for c in chars {
            polymer.insert(idx, Unit::new(c, idx, idx + 1));
            idx += 1;
        }
        Self { polymer, start: 0 }
    }

    fn react(&mut self) {
        let mut current_length = self.polymer.len();

        loop {
            self.react_loop();
            let new_length = self.length();
            if new_length == current_length {
                break;
            }
            current_length = new_length;
        }
    }

    fn get(&self, idx: &u32) -> Option<Unit> {
        match self.polymer.get(idx) {
            Some(unit) => Some(*unit),
            None => None,
        }
    }

    fn remove(&mut self, idx: &u32) {
        self.polymer.remove(idx);
    }

    fn react_loop(&mut self) {
        let mut current: Option<Unit> = self.get(&self.start);
        let mut last: Option<Unit> = None;

        loop {
            match current {
                Some(this) => {
                    let match_next = self
                        .get(&this.next)
                        .map_or(false, |f| this.characters_match(&f));

                    if match_next {
                        match last {
                            Some(last_unit) => {
                                let next =
                                    self.get(&this.next).map_or(last_unit.next + 1, |f| f.next);
                                self.polymer.insert(last_unit.id, last_unit.new_next(next));
                            }
                            None => {
                                self.start = self
                                    .polymer
                                    .get(&this.next)
                                    .map_or(self.start + 1, |f| f.next)
                            }
                        }
                        self.remove(&this.id);
                        self.remove(&this.next);
                        break;
                    };

                    last = current;
                    current = self.get(&this.next)
                }
                None => break,
            };
        }
    }

    fn length(&self) -> usize {
        self.polymer.len()
    }
}

impl Unit {
    fn new(character: char, id: u32, next: u32) -> Self {
        Self {
            character,
            id,
            next,
        }
    }

    fn new_next(&self, next: u32) -> Unit {
        Unit::new(self.character, self.id, next)
    }

    fn characters_match(&self, next: &Unit) -> bool {
        (self.character.is_ascii_lowercase() && next.character.is_ascii_uppercase()
            || self.character.is_ascii_uppercase() && next.character.is_ascii_lowercase())
            && self.character.to_ascii_lowercase() == next.character.to_ascii_lowercase()
    }
}
