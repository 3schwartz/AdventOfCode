use std::{fs};

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

    let result = first.react();

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

struct ReactState{
    first: Option<Box<Unit>>,
    last: Option<Box<Unit>>,
    current: Option<Box<Unit>>
}

impl Unit {
    fn new (character: char) -> Box<Self> {
        Box::new(Self {
            character, 
            next: None })
    }

    fn append(self: &mut Box<Self>, character: char) {
        let mut temp_next = self;

        while let Some(ref mut next) = temp_next.next
        {
            temp_next = next;
        }

        temp_next.next = Some(Unit::new(character));
    }

    fn match_next(&self, next : Option<Box<Unit>>) -> (Option<Box<Unit>>, bool) {
        match next {
            Some(other) if self.characters_match(&other) => {
                return (other.next, true)
            },
            Some(other) => (Some(other), false),
            None => (None, false),
        }
    }

    fn react(self: Box<Self>) -> Box<Unit> {
        let mut last_length = self.get_length();
        let mut to_evaluate = self;
        loop {
            to_evaluate = to_evaluate.react_loop();
            let new_length = to_evaluate.get_length();
            if new_length == last_length {
                break;
            }
        };
        return to_evaluate;
    }

    fn react_loop(self: Box<Self>) -> Box<Unit> {
        let mut last_unit: Option<&Box<Unit>> = None;
        let first = self;
        
        loop {
            let mut to_evaluate = match last_unit {
                Some(unit) => unit,
                None => &first,
            };
            let next_options = to_evaluate.next.take();

            match to_evaluate.match_next(next_options) {
                (Some(next), true) => {
                    let to_return: Box<Unit> = match last_unit {
                        Some(last) => {
                            last.next = Some(next);
                            first
                        },
                        None => next,
                    };
                    return to_return;
                },
                (Some(next), false) => {
                    to_evaluate.next = Some(next);
                    last_unit = Some(to_evaluate);
                },
                (None, _) => return first,
            }
        }
        // let match_next = self.match_next(self.next);

        // let next_temp = self.next.take();
        // match next_temp {
        //     Some(next) if (self.characters_match(&next)) => {
        //         match next.next {
        //             Some(new_next) => (new_next.react().0, true),
        //             None => (None, true),
        //         }
        //     },
        //     Some(next) => {
        //         let new_next = next.react();
        //         self.next = new_next.0;
                
        //         match new_next.1 {
        //             true => self.react(),
        //             false => (Some(Box::new(self)), false),
        //         }
        //     },
        //     None => (Some(Box::new(self)), false)
        // }
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
