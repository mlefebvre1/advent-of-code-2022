mod definitions;
mod map;

use std::fs;

use definitions::{Beacon, Sensor};

use definitions::Point;
use map::{manhattan_distance, Position};

fn main() -> anyhow::Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

#[allow(non_upper_case_globals)]
fn first() -> anyhow::Result<String> {
    let data = fs::read_to_string("day15/data/day15.txt")?;
    let mut m = map::Map::new(&data)?;
    const y: isize = 2000000;
    for pair in m.devices.0.iter() {
        let (sensor, beacon) = (&pair.0, &pair.1);
        let manhattan_dist = manhattan_distance(&sensor.0, &beacon.0);

        let (xmin, xmax) = (
            sensor.0.x - manhattan_dist as isize,
            sensor.0.x + manhattan_dist as isize,
        );
        for x in xmin..xmax {
            let xi = (x - m.offset()) as usize;
            if *sensor == Sensor::from((x, y)) {
                m.lane[xi] = Position::Sensor(sensor.clone());
            }
            if *beacon == Beacon::from((x, y)) {
                m.lane[xi] = Position::Beacon(beacon.clone());
            } else if m.lane[xi] == Position::Unknown {
                // Check if the manhattan distance between the current position and
                // the sensor is <= to manhattan distance between the sensor and the beacon.
                let cur_manhattan_dist = manhattan_distance(&Point { x, y }, &sensor.0);
                if cur_manhattan_dist <= manhattan_dist {
                    m.lane[xi] = Position::Nothing;
                }
            }
        }
    }
    let ans = m
        .lane
        .iter()
        .filter(|&e| matches!(*e, Position::Sensor(_) | Position::Nothing))
        .count();

    Ok(ans.to_string())
}

fn second() -> anyhow::Result<String> {
    Ok("".to_string())
}
