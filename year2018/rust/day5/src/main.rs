use std::{fs, rc::Rc, cell::{RefCell, Ref}};

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
        let mut last_unit: Option<Rc<RefCell<Unit>>> = None;
        let first = Rc::clone(&to_evaluate);
        let current = Rc::clone(&to_evaluate);
 
        loop {
            let loop_current = Rc::clone(&current);
            match Unit::match_next(loop_current) {
                (Some(next), true) => {
                    let to_return: Rc<RefCell<Unit>> = match last_unit {
                        Some(last) => {
                            let borrow = last.borrow_mut();
                            borrow.next = Some(next);
                            first
                        },
                        None => next,
                    };
                    return to_return;
                },
                (Some(next), false) => {
                    let borrow = loop_current.borrow_mut();
                    borrow.next = Some(next);
                    last_unit = Some(loop_current);
                },
                (None, _) => return first,
            }
        }
    }

    fn match_next(current : Rc<RefCell<Self>>) -> (Option<Rc<RefCell<Unit>>>, bool) {
        let foo = Rc::clone(&current);
        let c = foo.borrow();
        match c.next {
            Some(ref other) => {
                let bar = Rc::clone(other);
                if Unit::characters_match(c, &bar) {
                    let next = bar.borrow().next;
                    return (next, true)
                }
                return (Some(bar), false);
            },
            None => (None, false),
        }
    }

    fn characters_match(current : Ref<Self>, other: &Rc<RefCell<Self>>) -> bool {
        let o = other.borrow();
        (o.character.is_ascii_lowercase() && current.character.is_ascii_uppercase() ||
        o.character.is_ascii_uppercase() && current.character.is_ascii_lowercase()) &&
        o.character.to_ascii_lowercase() == current.character.to_ascii_lowercase()
    }

    fn get_length(&self) -> i32 {
        let sum = match &self.next {
            Some(next) => {
                next.borrow().get_length() + 1
            },
            None => 1,
        };
        sum
    }
}
