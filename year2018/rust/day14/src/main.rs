use std::collections::HashMap;
use anyhow::{anyhow, Result};

fn generate_recipe(initial: &str) -> Result<Vec<u128>> {
    let mut recipes = vec![];
    for c in initial.chars() {
        let recipe = c
            .to_digit(10)
            .ok_or_else(|| anyhow!("not able to parse: {}", c))? as u128;
        recipes.push(recipe);
    }
    Ok(recipes)
}

fn cook(
    first_elf: u128,
    second_elf: u128,
    recipes: &mut HashMap<u128, u128>,
    mut next_id: u128,
) -> Result<(u128, u128, u128)> {
    let first = *recipes
        .get(&first_elf)
        .ok_or_else(|| anyhow!("issue get: {} on: {:?}", first_elf, recipes))?;
    let second = *recipes
        .get(&second_elf)
        .ok_or_else(|| anyhow!("issue get: {} on: {:?}", second_elf, recipes))?;

    let new_recipe = first + second;

    if new_recipe > 9 {
        recipes.insert(next_id, 1);
        next_id += 1;
    }
    recipes.insert(next_id, new_recipe % 10);
    next_id += 1;

    let first_elf_new = (first_elf + first + 1) % recipes.len() as u128;
    let second_elf_new = (second_elf + second + 1) % recipes.len() as u128;

    Ok((first_elf_new, second_elf_new, next_id))
}

fn part_1(input_value: usize, initial: &str) -> Result<String> {
    let mut recipes = HashMap::new();
    for (idx, r) in generate_recipe(initial)?.iter().enumerate() {
        recipes.insert(idx as u128, *r);
    }
    let mut first_elf = 0;
    let mut second_elf = 1;
    let mut next_id = 2;

    while 10 + input_value >= recipes.len() {
        (first_elf, second_elf, next_id) = cook(first_elf, second_elf, &mut recipes, next_id)?;
    }

    let mut final_recipe = vec![];
    for i in (input_value)..(input_value + 10) {
        let f = recipes
            .get(&(i as u128))
            .ok_or_else(|| anyhow!("not able to get idx"))?
            .to_string();
        final_recipe.push(f);
    }

    let skip = final_recipe.join("");

    Ok(skip)
}

fn part_2(final_r: &str, initial: &str) -> Result<usize> {
    let mut recipes = HashMap::new();
    for (idx, r) in generate_recipe(initial)?.iter().enumerate() {
        recipes.insert(idx as u128, *r as u128);
    }
    let final_recipe = generate_recipe(final_r)?;

    let mut first_elf = 0;
    let mut second_elf = 1;
    let mut next_id = 2;

    loop {
        (first_elf, second_elf, next_id) =
            cook(first_elf, second_elf, &mut recipes, next_id)?;
        
        if recipes.len() < final_recipe.len() {
            continue;
        }

        let offset = recipes.len()-final_recipe.len();
        let mut done = true;
        for i in 0..final_recipe.len() {
            let x = recipes.get(&((i+offset) as u128)).ok_or_else(|| anyhow!("foo"))?;
            let y = final_recipe.get(i).ok_or_else(|| anyhow!("should be within"))?;
            if x != y {
                done = false;
            }
        }

        if done {
            break;
        }
    }

    Ok(recipes.len() - final_recipe.len())
}

fn main() -> Result<()> {
    let input = "084601";
    let input_value: usize = input.parse()?;
    let initial = "37";

    let part_1 = part_1(input_value, initial)?;
    println!("Part 1: {}", part_1);

    let part_2 = part_2(input, initial)?;
    println!("Part 2: {}", part_2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let initial = "37";

        let expected = vec![("51589", 9),
         ("01245", 5),
         ("92510", 18),
         ("59414", 2018)];

         // Act
         for (x, y) in expected {
            let part_2 = part_2(x, initial)?;
            assert_eq!(part_2, y)
         };

         Ok(())
    }
}