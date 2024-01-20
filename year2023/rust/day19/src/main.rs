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

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;
    let sections: Vec<&str> = input.split("\n\n").collect();

    let workflows = get_workflow(sections[0]);
    let mut accepted_sum = 0;
    for p in sections[1].lines() {
        let part = get_part(p);

        if is_accepted(&part, &workflows) {
            accepted_sum += part.values().sum::<u32>();
        }
    }
    println!("Part 1: {}", accepted_sum);

    Ok(())
}
