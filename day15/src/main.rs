mod definitions;
use definitions::{Beacon, Sensor};

use std::{fs, str::FromStr};

fn main() -> anyhow::Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

fn first() -> anyhow::Result<String> {
    let data = fs::read_to_string("day15/data/day15_easy.txt")?;
    for line in data.lines() {
        let mut l = line.split(':');
        let sensor = Sensor::from_str(l.next().unwrap())?;
        let beacon = Beacon::from_str(l.next().unwrap())?;
        println!("sensor={sensor:?}");
        println!("beacon={beacon:?}");
    }
    Ok("".to_string())
}

fn second() -> anyhow::Result<String> {
    Ok("".to_string())
}
