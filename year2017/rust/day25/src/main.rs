use std::collections::HashMap;
use std::str::FromStr;
use std::{collections::HashSet, fs};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day25_data.txt")?;

    let mut parts = input.trim().split("\n\n");
    let initial = parts
        .next()
        .ok_or_else(|| anyhow!("not able to get initial: {}", input))?;

    let (mut state, checksum) = parse_initial(initial)?;

    let states: HashMap<char, State> = parts
        .map(State::from_str)
        .collect::<Result<Vec<State>, _>>()?
        .into_iter()
        .map(|s| (s.state, s))
        .collect();

    let mut cursor = 0;
    let mut tape = HashSet::new();
    for _ in 0..checksum {
        let cursor_state = states
            .get(&state)
            .ok_or_else(|| anyhow!("missing state: {}", state))?;
        let action = if tape.contains(&cursor) {
            &cursor_state.exists
        } else {
            &cursor_state.not_exists
        };
        if action.to_write {
            tape.insert(cursor);
        } else {
            tape.remove(&cursor);
        }
        cursor += action.to_move;
        state = action.to_state;
    }

    println!("Part 1: {}", tape.len());
    Ok(())
}

impl State {
    fn _state_id(state_part: &str) -> Result<char> {
        let to_continue = state_part
            .trim()
            .trim_start_matches("In state ")
            .trim_end_matches(':');

        let chars: Vec<char> = to_continue.chars().collect();
        if chars.len() != 1 {
            return Err(anyhow!(
                "can also handle single char states when mapping state."
            ));
        }

        Ok(chars[0])
    }
}

struct StateAction {
    to_write: bool,
    to_move: i32,
    to_state: char,
}

struct State {
    state: char,
    exists: StateAction,
    not_exists: StateAction,
}

impl StateAction {
    fn new(write_part: &str, move_part: &str, continue_part: &str) -> Result<Self> {
        let to_write = write_part
            .trim()
            .trim_start_matches("- Write the value ")
            .trim_end_matches('.')
            == "1";

        let to_move_part: &str = move_part
            .trim()
            .trim_start_matches("- Move one slot to the ")
            .trim_end_matches('.');
        let to_move = match to_move_part {
            "right" => 1,
            "left" => -1,
            _ => return Err(anyhow!("not able to match move: {}", to_move_part)),
        };

        let to_continue = continue_part
            .trim()
            .trim_start_matches("- Continue with state ")
            .trim_end_matches('.');

        let chars: Vec<char> = to_continue.chars().collect();
        if chars.len() != 1 {
            return Err(anyhow!("can only handle single char states."));
        }

        Ok(Self {
            to_write,
            to_move,
            to_state: chars[0],
        })
    }
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != 9 {
            return Err(anyhow!("state had wrong lenght: {}", s));
        }

        let state = Self::_state_id(lines[0])?;

        let not_exists = StateAction::new(lines[2], lines[3], lines[4])?;
        let exists = StateAction::new(lines[6], lines[7], lines[8])?;

        Ok(Self {
            state,
            not_exists,
            exists,
        })
    }
}

fn parse_initial(input: &str) -> Result<(char, i32)> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() != 2 {
        return Err(anyhow!("initial had wrong lenght: {}", input));
    }

    let initial_state = lines[0]
        .trim()
        .trim_start_matches("Begin in state ")
        .trim_end_matches('.');

    let chars: Vec<char> = initial_state.chars().collect();
    if chars.len() != 1 {
        return Err(anyhow!("initial state not char."));
    }

    let checksum: i32 = lines[1]
        .trim()
        .trim_start_matches("Perform a diagnostic checksum after ")
        .trim_end_matches(" steps.")
        .parse()?;

    Ok((chars[0], checksum))
}
