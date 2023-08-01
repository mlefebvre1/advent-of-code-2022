mod definitions;
mod map;

use definitions::Point;
use definitions::{Beacon, Sensor};
use map::{manhattan_distance, DevicePairs, Position};
use num::{BigInt, FromPrimitive};
use std::fs;
use std::ops::Range;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

#[allow(non_upper_case_globals)]
fn first() -> anyhow::Result<String> {
    let data = fs::read_to_string("day15/data/day15.txt")?;
    let mut m = map::Map::new(&data, None)?;
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
    let data = fs::read_to_string("day15/data/day15.txt")?;
    let ans = find_frequency_tuning(&data).unwrap();
    Ok(ans.to_string())
}

fn find_frequency_tuning(data: &str) -> Option<BigInt> {
    const M: usize = 4000000;
    let pairs = DevicePairs::from_str(data).unwrap();
    for y in 0..M {
        let mut ranges = Vec::new();
        for pair in pairs.0.iter() {
            let sensor = &pair.0;
            let beacon = &pair.1;
            let manhattan_dist = manhattan_distance(&sensor.0, &beacon.0);
            let xspan = manhattan_dist as isize - (sensor.0.y - y as isize).abs();
            if xspan > 0 {
                let xmin = std::cmp::max(sensor.0.x - xspan, 0);
                let xmax = std::cmp::min(sensor.0.x + xspan, M as isize);
                let r = xmin..xmax;
                ranges.push(r);
            }
            ranges.sort_by_key(|k| k.start);
        }
        if ranges.len() >= 2 {
            if let Some(x) = merge_ranges(ranges) {
                let mut ans = BigInt::from_usize(y).unwrap();
                ans += (x as usize) * M;
                return Some(ans);
            }
        }
    }
    None
}

fn merge_ranges(ranges: Vec<Range<isize>>) -> Option<isize> {
    let mut current = ranges.first().unwrap().clone();
    for range in ranges.iter().skip(1) {
        if range.start <= current.end + 1 {
            current.start = std::cmp::min(current.start, range.start);
            current.end = std::cmp::max(current.end, range.end);
        } else {
            return Some(current.end + 1);
        }
    }
    None
}
