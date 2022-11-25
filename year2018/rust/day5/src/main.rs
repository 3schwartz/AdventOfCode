use std::{fs, rc::Rc, cell::{RefCell}};

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
        Unit::append(&first, c);
    }

    let result = Unit::react(first);

    let length = result.borrow().get_length();
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
    next: Option<Rc<RefCell<Unit>>>,
}

impl Unit {
    fn new (character: char) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            character, 
            next: None }))
    }

    fn append(first : &Rc<RefCell<Self>>, character: char) {
        let mut temp_next = first;

        while let Some(ref next) = temp_next.borrow().next
        {
            temp_next = next;
        }
        let borrowed = temp_next.borrow_mut();

        borrowed.next = Some(Unit::new(character));
    }

    fn match_next(self: Rc<Self>) -> (Option<Rc<Unit>>, bool) {
        match self.next {
            Some(other) if self.characters_match(&other) => {
                return (other.next, true)
            },
            Some(other) => (Some(other), false),
            None => (None, false),
        }
    }

    fn react(initial: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let mut last_length : i32;
        {
            last_length = initial.borrow().get_length();
        };

        let mut to_evaluate = initial;
        loop {
            to_evaluate = Unit::react_loop(to_evaluate);
            let mut new_length : i32;
            {
                new_length = to_evaluate.borrow().get_length();
            }
            if new_length == last_length {
                break;
            }
            last_length = new_length;
        };
        return to_evaluate;
    }

    fn react_loop(to_evaluate: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let mut last_unit: Option<Rc<Unit>> = None;
        let first = Rc::clone(&to_evaluate);
        let current = Rc::clone(&to_evaluate);
 
        loop {
            // let to_evaluate = match &last_unit {
            //     Some(unit) => unit,
            //     None => &Rc::clone(&first),
            // };
            let loop_current = Rc::clone(&current);
            match loop_current.match_next() {
                (Some(next), true) => {
                    let to_return: Rc<Unit> = match last_unit {
                        Some(last) => {
                            last.next = Some(next);
                            first
                        },
                        None => next,
                    };
                    return to_return;
                },
                (Some(next), false) => {
                    loop_current.next = Some(next);
                    last_unit = Some(loop_current);
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
