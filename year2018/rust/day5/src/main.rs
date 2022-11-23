use std::{cell::RefCell, rc::{Weak, Rc}};

fn main() {
    let input = "dabAcCaCBAcCcaDA";
    let mut chars = input.chars();
    let c = chars.next().unwrap();
    
    let mut polymer = Polymer::new();
    polymer.insert_first(Unit::new(c));

    for c in chars {
        // previous = previous.add_next(c);
        // println!("{}", previous.character)
    }

}
// type UnitRef = Rc<RefCell<Unit>>;
// type UnitWeakRef = Weak<RefCell<Unit>>;

struct Polymer {
    first: Option<Rc<RefCell<Unit>>>
}

impl Polymer {
    fn new() -> Self {
        Self {
            first: None
        }
    }

    fn insert_first(&mut self, unit: Rc<RefCell<Unit>>) {
        self.first = Some(unit)
    }

    fn insert(&self, character: char, unit: &Rc<RefCell<Unit>>) {
        let next_unit = Unit::new_with_previous(character, unit);
        let mut previous_unit = unit.borrow_mut();
        previous_unit.next = Some(next_unit);
    }
}

struct Unit {
    character: char,
    previous: Option<Weak<RefCell<Unit>>>,
    next: Option<Rc<RefCell<Unit>>>,
}

impl Unit {
    fn new(character: char) -> Rc<RefCell<Unit>> {
        Rc::new(RefCell::new(Self { 
            character,
            previous: None,
            next: None
        }))
    }

    fn new_with_previous(character: char, previous: &Rc<RefCell<Unit>>) -> Rc<RefCell<Unit>> {
        Rc::new(RefCell::new(Self { 
            character,
            previous: Some(Rc::<RefCell<Unit>>::downgrade(previous)),
            next: None
        }))
    }
}

