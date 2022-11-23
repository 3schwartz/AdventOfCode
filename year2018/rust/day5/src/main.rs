use std::{cell::RefCell, rc::{Weak, Rc}, borrow::{BorrowMut}};

fn main() {
    let input = "dabAcCaCBAcCcaDA";
    let mut chars = input.chars();
    let c = chars.next().unwrap();
    
    let mut polymer = Polymer::new(Unit::new(c));

    for c in chars {
        polymer.insert(c)
    }

    polymer.react();

}
// type UnitRef = Rc<RefCell<Unit>>;
// type UnitWeakRef = Weak<RefCell<Unit>>;

struct Polymer {
    root: Rc<RefCell<Unit>>,
    previous: Rc<RefCell<Unit>>
}

impl Polymer {
    fn new(first: Rc<RefCell<Unit>>) -> Self {
        Self {
            previous: first.clone(),
            root: first,
        }
    }

    fn insert(&mut self, character: char) {
        let next_unit = Unit::new_with_previous(character, &self.previous);
        {
            let mut previous_unit = self.previous.borrow();
            previous_unit.next= Some(next_unit.clone());
        }
        self.previous = next_unit;
    }

    fn react(&self) {
        let root_borrowed = self.root.borrow();
        let length = root_borrowed.get_length();
        root_borrowed.react();
    }
}

struct Unit {
    character: char,
    previous: Option<Rc<RefCell<Unit>>>,
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
            previous: Some(previous.clone()),
            next: None
        }))
    }

    fn react(&mut self) {
        match &self.next {
            Some(next) => {
                let next_borrow = next.borrow();

                if (next_borrow.character.is_lowercase() && self.character.is_uppercase() ||
                    next_borrow.character.is_uppercase() && self.character.is_lowercase()) &&
                    self.character.to_lowercase().eq(next_borrow.character.to_lowercase()) {
                    
                    match &mut self.previous {
                        Some(previous) => {
                            let mut previous_borrow = previous.borrow();
                            previous_borrow.next = self.next;
                        },
                        None => todo!(),
                    }
                }
            },
            None => return,
        }
    }

    fn get_length(&self) -> i32 {
        let sum = match &self.next {
            Some(next) => {
                let next_borrowed = next.borrow();
                next_borrowed.get_length() + 1
            },
            None => 1,
        };
        sum
    }
}

