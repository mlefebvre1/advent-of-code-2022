use anyhow::Result;
use std::collections::HashSet;
use std::fs;

#[allow(dead_code)]
fn first() -> Result<String> {
    let data = fs::read_to_string("src/days/data/day3.txt")?;
    let ans: usize = data
        .lines()
        .map(|line| {
            let compartments = split_in_two_compartments(line);
            let items_in_both = get_items_appearing_on_both(compartments);
            get_items_priority_total(&items_in_both)
        })
        .sum();
    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let data = fs::read_to_string("src/days/data/day3.txt")?;
    let lines: Vec<&str> = data.lines().collect();
    let ans: usize = lines
        .chunks(3)
        .map(|compartments| {
            let items = get_items_appearing_on_3(compartments);
            get_items_priority_total(&items)
        })
        .sum();

    Ok(ans.to_string())
}

fn split_in_two_compartments(line: &str) -> (Vec<char>, Vec<char>) {
    let nb_items_per_compartments = line.chars().count() / 2;
    let mut items = line.chars();
    let items = items.by_ref();
    let first_compartment = items.take(nb_items_per_compartments).collect();
    let second_compartment = items.take(nb_items_per_compartments).collect();
    (first_compartment, second_compartment)
}

fn get_items_appearing_on_both(compartments: (Vec<char>, Vec<char>)) -> HashSet<char> {
    let mut items = HashSet::new();
    for c in compartments.0.iter() {
        if compartments.1.contains(c) {
            items.insert(*c);
        }
    }
    items
}

fn get_items_appearing_on_3(compartments: &[&str]) -> HashSet<char> {
    let mut items = HashSet::new();
    for c in compartments[0].chars() {
        if compartments[1].contains(c) && compartments[2].contains(c) {
            items.insert(c);
        }
    }
    items
}

fn get_items_priority_total(items: &HashSet<char>) -> usize {
    items
        .iter()
        .map(|&item| {
            if item.is_uppercase() {
                item as usize - 38 // 'A'=65 and we rebase to 27 so.. 65-27
            } else {
                item as usize - 96 // 'a' = 97 and we rebase to 1 so 97-1
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part1() {
        println!("Day3 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_part2() {
        println!("Day3 - Second problem : {}", second().unwrap());
    }
}
