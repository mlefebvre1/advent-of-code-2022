mod cave;
mod model;

use cave::Cave;
use model::action::JetPattern;

fn main() -> anyhow::Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

#[allow(non_upper_case_globals)]
fn first() -> anyhow::Result<String> {
    let data = std::fs::read_to_string("day17/data/day17.txt")?;
    let patterns: Vec<JetPattern> = data.chars().map(|c| c.try_into().unwrap()).collect();
    let mut cave = Cave::new(patterns);
    let ans = cave.run(2022) - 1;

    Ok(ans.to_string())
}

fn second() -> anyhow::Result<String> {
    let data = std::fs::read_to_string("day17/data/day17.txt")?;
    Ok("".to_string())
}
