use anyhow::{Error, Result};
use std::{collections::HashSet, fs, str::FromStr};

#[allow(dead_code)]
fn first() -> Result<String> {
    let data = fs::read_to_string("src/days/data/day4.txt")?;
    let ans: usize = data
        .lines()
        .map(|line| make_sets_from_ranges(line).unwrap())
        .filter(|(set1, set2)| set1.is_subset(set2) || set2.is_subset(set1))
        .count();

    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let data = fs::read_to_string("src/days/data/day4.txt")?;
    let ans: usize = data
        .lines()
        .map(|line| make_sets_from_ranges(line).unwrap())
        .filter(|(set1, set2)| !set1.is_disjoint(set2))
        .count();
    Ok(ans.to_string())
}

#[derive(Debug)]
struct SectionRange {
    start: usize,
    end: usize,
}
impl FromStr for SectionRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts.next().unwrap().parse()?;
        let end = parts.next().unwrap().parse()?;
        Ok(Self { start, end })
    }
}

fn make_sets_from_ranges(line: &str) -> Result<(HashSet<usize>, HashSet<usize>)> {
    let mut ranges = line.split(',');
    let range1 = SectionRange::from_str(ranges.next().unwrap())?;
    let range2 = SectionRange::from_str(ranges.next().unwrap())?;
    let set1: HashSet<usize> = (range1.start..=range1.end).collect();
    let set2: HashSet<usize> = (range2.start..=range2.end).collect();
    Ok((set1, set2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part1() {
        println!("Day4 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_part2() {
        println!("Day4 - Second problem : {}", second().unwrap())
    }
}
