use std::fs;

fn main() {
    let input = fs::read_to_string("../../data/day5_data.txt")
        .expect("couldn't open file");
    let length = get_polymer_length(input.trim());
    println!("Part 1: {}", length)
}

fn get_polymer_length(polymer: &str) -> i32 {
    let mut chars = polymer.chars();
    let c = chars.next().unwrap();
    
    let mut first = Unit::new(c);

    for c in chars {
        first.append(c);
    }

    let result = match first.react(){
        (None, _) => panic!("Didn't work"),
        (Some(unit), _) => unit,
    };

    let length = result.get_length();
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
    next: Option<Box<Unit>>,
}

impl Unit {
    fn new (character: char) -> Box<Self> {
        Box::new(Self {
            character, next: None })
    }

    fn append(self: &mut Box<Self>, character: char) {
        let mut temp_next = self;

        while let Some(ref mut next) = temp_next.next
        {
            temp_next = next;
        }

        temp_next.next = Some(Unit::new(character));
    }

    fn react(mut self) -> (Option<Box<Unit>>, bool) {
        let next_temp = self.next.take();
        match next_temp {
            Some(next) if (self.characters_match(&next)) => {
                match next.next {
                    Some(new_next) => (new_next.react().0, true),
                    None => (None, true),
                }
            },
            Some(next) => {
                let new_next = next.react();
                self.next = new_next.0;
                
                match new_next.1 {
                    true => self.react(),
                    false => (Some(Box::new(self)), false),
                }
            },
            None => (Some(Box::new(self)), false)
        }
    }

    fn characters_match(&self, other : &Unit) -> bool {
        (other.character.is_ascii_lowercase() && self.character.is_ascii_uppercase() ||
        other.character.is_ascii_uppercase() && self.character.is_ascii_lowercase()) &&
            self.character.to_ascii_lowercase() == other.character.to_ascii_lowercase()
    }

    fn get_length(&self) -> i32 {
        let sum = match &self.next {
            Some(next) => {
                next.get_length() + 1
            },
            None => 1,
        };
        sum
    }
}
