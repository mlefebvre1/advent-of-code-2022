use std::{fs, str::FromStr};

use anyhow::anyhow;

#[allow(dead_code)]
fn first() -> anyhow::Result<String> {
    let data = fs::read_to_string("src/days/data/day15_easy.txt")?;
    for line in data.lines() {
        let mut l = line.split(':');
        let sensor = Sensor::from_str(l.next().unwrap())?;
        let beacon = Beacon::from_str(l.next().unwrap())?;
        println!("sensor={sensor:?}");
        println!("beacon={beacon:?}");
    }
    Ok("".to_string())
}

#[allow(dead_code)]
fn second() -> anyhow::Result<String> {
    Ok("".to_string())
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor(Point);
#[derive(Debug)]
struct Beacon(Point);

impl FromStr for Sensor {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("Sensor") {
            let s = s.strip_prefix("Sensor at").unwrap();
            let mut coord = s.split(',');
            let x = get_coord_value(&mut coord)?;
            let y = get_coord_value(&mut coord)?;

            Ok(Self(Point { x, y }))
        } else {
            Err(anyhow!("Expected string to start with Sensor"))
        }
    }
}

impl FromStr for Beacon {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix(" closest beacon is at") {
            let mut coord = s.split(',');
            let x = get_coord_value(&mut coord)?;
            let y = get_coord_value(&mut coord)?;
            Ok(Self(Point { x, y }))
        } else {
            Err(anyhow!("bad prefix for breacon"))
        }
    }
}

fn get_coord_value<'a, I, O, E>(coord: &mut I) -> Result<O, E>
where
    I: Iterator<Item = &'a str>,
    O: num::Integer + FromStr + FromStr<Err = E>,
{
    coord
        .next()
        .unwrap()
        .trim()
        .split("=")
        .nth(1)
        .unwrap()
        .parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_part() {
        println!("Day14 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_second_part() {
        println!("Day14 - Second problem : {}", second().unwrap())
    }
}
