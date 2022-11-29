use std::{collections::{HashMap, HashSet}, fs::File, io::Write};

struct Unit {
    character: char,
    id: u32,
    next: u32,
}

pub struct PolymerImprover{
    initial_polymer: String,
    unique_units: HashSet<char>
}

impl PolymerImprover{
    pub fn new(input: String) -> Self {
        let mut chars_unique = HashSet::new();
        for c in input.chars() {
            chars_unique.insert(c);
        }
        Self {
            initial_polymer: input,
            unique_units: chars_unique
        }
    }

    pub fn find_polymer_length(&self) -> usize {
        let mut min_length = usize::MAX;
        for c in &self.unique_units {
            let temp = self.initial_polymer
                .replace(&[c.to_ascii_lowercase(), c.to_ascii_uppercase()], "");
            let mut polymer = Polymer::new(&temp);
            let length = polymer.find_polymer_length();
            if length < min_length {
                min_length = length;
            };
        }
        min_length
    }
}

pub struct Polymer{
    polymer: HashMap<u32,Unit>,
    start: u32
}

impl Polymer{
    pub fn new (input : &str) -> Polymer {
        let chars = input.chars();
        let mut polymer: HashMap<u32, Unit> = HashMap::new();
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

    pub fn write_to_file(&self, path: &str) {
        let mut file = File::create(path).expect("couldn't write to file");
        let mut idx = self.start;
        while let Some(unit) = self.get(&idx) {
            write!(file, "{}", unit.character).expect("Unable to write");
            idx = unit.next;
        }
    }

    pub fn find_polymer_length(&mut self) -> usize {

        self.react();
    
        let length = self.length();
        length
    }

    fn react(&mut self) {
        let mut current_length = self.polymer.len();

        loop {
            let to_remove = self.react_loop();
            for id in to_remove {
                self.remove(&id);
            }
            let new_length = self.length();
            if new_length == current_length {
                break;
            }
            current_length = new_length;
        }
    }

    fn get(&self, idx: &u32) -> Option<&Unit> {
        self.polymer.get(idx)
    }

    fn remove(&mut self, idx: &u32) {
        self.polymer.remove(idx);
    }

    fn react_loop(&mut self) -> Vec<u32> {
        let mut current = self.polymer.get(&self.start);
        let mut last : Option<&Unit> = None;
        let mut to_remove = Vec::<u32>::new();

        loop {
            match current {
                Some(this) => {
                    let match_next = self.get(&this.next)
                        .map_or(false, |f| this.characters_match(f));
                    
                    if match_next {
                        to_remove.push(this.id);
                        to_remove.push(this.next);
                        match last {
                            Some(last_unit) => {
                                let next = self.polymer.get(&this.next)
                                    .map_or(last_unit.next + 1, |f| f.next);
                                self.polymer.insert(last_unit.id, last_unit.new_next(next));
                            },
                            None => {
                                self.start = self.polymer.get(&this.next)
                                    .map_or(self.start + 1, |f| f.next)
                            },
                        }
                        break;
                    };

                    last = current;
                    current = self.get(&this.next)
                },
                None => break,
            }
        };
        return to_remove;
    }

    fn length(&self) -> usize {
        self.polymer.len()
    }
}

impl Unit {
    fn new (character: char, id: u32, next: u32) -> Self {
        Self {
            character, id, next
        }
    }

    fn new_next(&self, next: u32) -> Unit {
        Unit::new(
            self.character, self.id, next
        )
    }

    fn characters_match(&self, next: &Unit) -> bool {
        (self.character.is_ascii_lowercase() && next.character.is_ascii_uppercase() ||
         self.character.is_ascii_uppercase() && next.character.is_ascii_lowercase()) &&
         self.character.to_ascii_lowercase() == next.character.to_ascii_lowercase()
    }
}
