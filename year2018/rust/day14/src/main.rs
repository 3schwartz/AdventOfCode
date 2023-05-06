use anyhow::{Result,anyhow};

fn part_1(input_value: usize, initial: &str) -> Result<String> {
    let mut recipes = generate_recipe(initial)?;
    let mut first_elf = 0;
    let mut second_elf = 1;

    while 10 + input_value >= recipes.len() {
        (first_elf, second_elf) = cook(first_elf, second_elf, &mut recipes)?;
    }

    let skip = recipes
        .iter()
        .skip(input_value)
        .take(10)
        .map(|n| n.to_string())
        .collect::<String>();

    Ok(skip)
}

fn generate_recipe(initial: &str) -> Result<Vec<usize>> {
    let mut recipes = vec![];
    for c in initial.chars() {
        let recipe = c.to_digit(10)
            .ok_or_else(|| anyhow!("not able to parse: {}", c))? as usize;
        recipes.push(recipe);
    }
    Ok(recipes)
}

fn part_2(final_r: &str, initial: &str) -> Result<usize> {
    let mut recipes = generate_recipe(initial)?;
    let final_recipe = generate_recipe(final_r)?;

    let mut first_elf = 0;
    let mut second_elf = 1;

    loop {
        (first_elf, second_elf) = cook(first_elf, second_elf, &mut recipes)?;
        let found = recipes
            .iter()
            .rev()
            .take(final_recipe.len())
            .rev()
            .eq(final_recipe.iter());
        if found {
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

fn cook(first_elf: usize, second_elf: usize, recipes: &mut Vec<usize>) -> Result<(usize, usize)> {
    let first = *recipes.get(first_elf)
        .ok_or_else(|| anyhow!("issue get: {} on: {:?}", first_elf, recipes))?;
    let second = *recipes.get(second_elf)
        .ok_or_else(|| anyhow!("issue get: {} on: {:?}", second_elf, recipes))?;

    let new_recipe = first + second;

    if new_recipe > 9 {
        recipes.push(1);
    }
    recipes.push(new_recipe % 10);
    
    let first_elf_new = (first_elf + first + 1) % recipes.len();
    let second_elf_new = (second_elf + second + 1) % recipes.len();

    Ok((first_elf_new, second_elf_new))

}
