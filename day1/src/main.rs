use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

fn first() -> Result<String> {
    let ans = elf_calories()?.into_iter().max().unwrap();
    Ok(ans.to_string())
}

fn second() -> Result<String> {
    let mut calories = elf_calories()?;
    calories.sort();
    let ans: usize = calories.iter().rev().take(3).sum();
    Ok(ans.to_string())
}

fn elf_calories() -> Result<Vec<usize>> {
    let data = fs::read_to_string("day1/data/day1.txt")?;
    Ok(data
        .lines()
        .scan(0_usize, |acc, line| {
            if let Ok(calories) = line.parse::<usize>() {
                *acc += calories;
                Some(0)
            } else {
                let total = *acc;
                *acc = 0;
                Some(total)
            }
        })
        .filter(|&x| x != 0)
        .collect())
}
