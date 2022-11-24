use std::{fs, ptr};

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

struct ReactState{
    first: Option<Box<Unit>>,
    last: Option<Box<Unit>>,
    current: Option<Box<Unit>>
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

    fn match_next(&self, next : Option<Box<Unit>>) -> (Option<Box<Unit>>, bool) {
        match next {
            Some(other) if self.characters_match(&other) => {
                return (other.next, true)
            },
            Some(other) => (Some(other), false),
            None => (None, false),
        }
    }

    fn react_loop(self: Box<Self>) -> Box<Unit> {
        let mut last_length = self.get_length();
        
        loop {

            let new_length = .get_length();
            if new_length == last_length {
                break;
            }
        }
    }

    fn react(self: &mut Box<Self>) -> (Option<Box<Unit>>, bool) {
        let mut temp_unit: Option<&Unit> = None;
        let mut last_unit: Option<(Box<Unit>, Box<Unit>)> = None;
        // let mut first_unit = self;
        let mut unit_to_evaluate = self;
        let mut is_first = true;
        
        loop {
            match unit_to_evaluate.match_next(unit_to_evaluate.next) {
                (Some(ref mut next), true) => {
                    match last_unit {
                        Some((first, last)) => {
                            last.next = Some(*next);
                            return 
                        },
                        None => todo!(),
                    }
                    last_unit = Some(&unit_to_evaluate);

                },
                (Some(next), false) => todo!(),
                (None, _) => todo!(),
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
