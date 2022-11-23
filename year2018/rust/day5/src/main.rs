

fn main() {
    let input = "dabAcCaCBAcCcaDA";
    let mut chars = input.chars();
    let c = chars.next().unwrap();
    
    let mut first = UnitP::new(c);

    for c in chars {
        first.append(c);
    }

    let result = match first.react(){
        (None, _) => panic!("Didn't work"),
        (Some(unit), _) => unit,
    };

    let length = result.get_length();

    println!("Part 1: {}", length)

}

struct UnitP {
    character: char,
    next: Option<Box<UnitP>>,
}

impl UnitP {
    fn new (character: char) -> Self {
        Self {
            character, next: None }
    }

    fn append(&mut self, character: char) {
        match &mut self.next {
            Some(next) => {
                next.append(character);
            },
            None => {
                self.next = Some(Box::new(UnitP::new(character)));
            },
        }
    }

    fn react(mut self) -> (Option<Box<UnitP>>, bool) {
        let next_temp = self.next;
        match next_temp {
            Some(next) if (self.characters_match(&next)) => {
                    match next.next {
                        Some(new_next) => (new_next.react().0, true),
                        None => (None, true),
                    }
                },
            Some(next) => {
                let foo = next.react();

                if foo.1 {
                    self.next = foo.0;
                    return self.react();
                }

                return (Some(Box::new(self)), false);


                // match next.react() {
                //     (new_next, true) => {
                //         self.next = new_next;
                //         return self.react();
                //     },
                //     (_, false) => {
                //         return (Some(Box::new(self)), false)
                //     }
                // }
            },
            None => (Some(Box::new(self)), false)
        }
    }

    fn characters_match(&self, other : &UnitP) -> bool {
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
