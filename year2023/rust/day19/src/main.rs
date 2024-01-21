use std::{collections::HashMap, fs};

use anyhow::Result;

fn get_part(mut input: &str) -> HashMap<&str, u32> {
    input = &input[1..input.len() - 1];
    let categories: Vec<&str> = input.split(',').collect();
    let mut part = HashMap::new();
    for categorie in categories {
        let splitted: Vec<&str> = categorie.split('=').collect();
        part.insert(splitted[0], splitted[1].parse().unwrap());
    }
    part
}

fn get_workflow(input: &str) -> HashMap<&str, &str> {
    let mut workflows = HashMap::new();
    for line in input.lines() {
        let splitted: Vec<&str> = line.split('{').collect();
        workflows.insert(splitted[0], &splitted[1][..splitted[1].len() - 1]);
    }
    workflows
}

fn is_accepted(part: &HashMap<&str, u32>, workflows: &HashMap<&str, &str>) -> bool {
    let mut workflow = "in";
    loop {
        let rules = *workflows.get(workflow).unwrap();
        for rule in rules.split(',') {
            let mut accepted = true;
            let mut next_workflow = rule;
            if rule.contains(':') {
                let rule_splitted: Vec<&str> = rule.split(':').collect();
                next_workflow = rule_splitted[1];
                let category = &rule_splitted[0][0..1];
                let operation = &rule_splitted[0][1..2];
                let value: u32 = rule_splitted[0][2..].parse().unwrap();
                if operation == ">" {
                    accepted = part[category] > value;
                } else {
                    accepted = part[category] < value;
                }
            }
            if accepted {
                if next_workflow == "A" {
                    return true;
                }
                if next_workflow == "R" {
                    return false;
                }
                workflow = next_workflow;
                break;
            }
        }
    }
}

fn find_accepted_in_parts(input: &str) -> u32 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let workflows = get_workflow(sections[0]);
    let mut accepted_sum = 0;
    for p in sections[1].lines() {
        let part = get_part(p);

        if is_accepted(&part, &workflows) {
            accepted_sum += part.values().sum::<u32>();
        }
    }
    accepted_sum
}

#[derive(Clone, Copy)]
struct Ranges {
    x_min: i64,
    x_max: i64,
    m_min: i64,
    m_max: i64,
    a_min: i64,
    a_max: i64,
    s_min: i64,
    s_max: i64,
}

impl Ranges {
    #[allow(clippy::too_many_arguments)]
    fn new(
        x_min: i64,
        x_max: i64,
        m_min: i64,
        m_max: i64,
        a_min: i64,
        a_max: i64,
        s_min: i64,
        s_max: i64,
    ) -> Self {
        Self {
            x_min,
            x_max,
            m_min,
            m_max,
            a_min,
            a_max,
            s_min,
            s_max,
        }
    }

    fn is_valid(&self) -> bool {
        self.x_min <= self.x_max
            && self.m_min <= self.m_max
            && self.a_min <= self.a_max
            && self.s_min <= self.s_max
    }

    fn total_accepted_in_ranges(&self) -> i64 {
        (self.x_max - self.x_min + 1)
            * (self.m_max - self.m_min + 1)
            * (self.a_max - self.a_min + 1)
            * (self.s_max - self.s_min + 1)
    }

    fn split_ranges(&self, category: &str, operation: &str, value: i64) -> (Ranges, Ranges) {
        (
            self.operation(category, true, operation, value),
            self.operation(category, false, operation, value),
        )
    }

    fn operation(&self, category: &str, within: bool, operation: &str, value: i64) -> Ranges {
        match category {
            "x" => {
                let (x_min, x_max) =
                    Ranges::update_range(operation, within, value, self.x_min, self.x_max);
                Ranges::new(
                    x_min, x_max, self.m_min, self.m_max, self.a_min, self.a_max, self.s_min,
                    self.s_max,
                )
            }
            "m" => {
                let (m_min, m_max) =
                    Ranges::update_range(operation, within, value, self.m_min, self.m_max);
                Ranges::new(
                    self.x_min, self.x_max, m_min, m_max, self.a_min, self.a_max, self.s_min,
                    self.s_max,
                )
            }
            "a" => {
                let (a_min, a_max) =
                    Ranges::update_range(operation, within, value, self.a_min, self.a_max);
                Ranges::new(
                    self.x_min, self.x_max, self.m_min, self.m_max, a_min, a_max, self.s_min,
                    self.s_max,
                )
            }
            "s" => {
                let (s_min, s_max) =
                    Ranges::update_range(operation, within, value, self.s_min, self.s_max);
                Ranges::new(
                    self.x_min, self.x_max, self.m_min, self.m_max, self.a_min, self.a_max, s_min,
                    s_max,
                )
            }
            _ => panic!("{}", category),
        }
    }

    fn update_range(operation: &str, within: bool, value: i64, min: i64, max: i64) -> (i64, i64) {
        match (operation, within) {
            (">", true) => (std::cmp::max(min, value + 1), max),
            ("<", true) => (min, std::cmp::min(max, value - 1)),
            ("<", false) => (std::cmp::max(min, value), max),
            (">", false) => (min, std::cmp::min(max, value)),
            _ => panic!("{}", operation),
        }
    }
}

#[derive(Clone)]
struct State<'a> {
    workflow: &'a str,
    ranges: Ranges,
}

impl<'a> State<'a> {
    fn new(workflow: &'a str, ranges: Ranges) -> Self {
        Self { workflow, ranges }
    }
}

fn find_accepted_in_workflows(input: &str) -> i64 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let workflows = get_workflow(sections[0]);

    let mut ranges = vec![State::new(
        "in",
        Ranges::new(1, 4_000, 1, 4_000, 1, 4_000, 1, 4_000),
    )];

    let mut accepted = 0;
    while let Some(state) = ranges.pop() {
        if state.workflow == "A" {
            accepted += state.ranges.total_accepted_in_ranges();
            continue;
        }
        if state.workflow == "R" {
            continue;
        }
        if !state.ranges.is_valid() {
            continue;
        }
        let mut current_ranges = state.ranges;
        let rules = *workflows.get(state.workflow).unwrap();
        for rule in rules.split(',') {
            let mut next_workflow = rule;

            if rule.contains(':') {
                let rule_splitted: Vec<&str> = rule.split(':').collect();
                next_workflow = rule_splitted[1];
                let category = &rule_splitted[0][0..1];
                let operation = &rule_splitted[0][1..2];
                let value: i64 = rule_splitted[0][2..].parse().unwrap();
                let (inside, outside) = current_ranges.split_ranges(category, operation, value);
                current_ranges = outside;
                ranges.push(State::new(next_workflow, inside));
            } else {
                ranges.push(State::new(next_workflow, current_ranges));
            }
        }
    }
    accepted
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;

    let accepted_sum = find_accepted_in_parts(&input);

    println!("Part 1: {}", accepted_sum);

    let part_2 = find_accepted_in_workflows(&input);

    println!("Part 2: {}", part_2);

    Ok(())
}
