use crate::definitions::{Beacon, Point, Sensor};
use anyhow::{Error, Result};
use std::{fmt::Display, str::FromStr};

pub fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    let (x1, x2) = (p1.x, p2.x);
    let (y1, y2) = (p1.y, p2.y);
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}

#[derive(Debug)]
pub struct DevicePair(pub Sensor, pub Beacon);

impl DevicePair {
    pub fn manhattan_distance(&self) -> usize {
        let (sensor, beacon) = (&self.0, &self.1);
        manhattan_distance(&sensor.0, &beacon.0)
    }

    pub fn x_min(&self) -> isize {
        let sensor = &self.0;
        sensor.0.x - self.manhattan_distance() as isize
    }
    pub fn x_max(&self) -> isize {
        let sensor = &self.0;
        sensor.0.x + self.manhattan_distance() as isize
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Position {
    Beacon(Beacon),
    Sensor(Sensor),
    Nothing, // Not beacon or sensor
    Unknown,
}
impl Default for Position {
    fn default() -> Self {
        Self::Unknown
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Beacon(_) => "B",
            Self::Sensor(_) => "S",
            Self::Nothing => "#",
            Self::Unknown => ".",
        };
        write!(f, "{s}")
    }
}

pub struct DevicePairs(pub Vec<DevicePair>);

impl FromStr for DevicePairs {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s
            .lines()
            .map(|line| {
                let mut l = line.split(':');
                let sensor = Sensor::from_str(l.next().unwrap()).unwrap();
                let beacon = Beacon::from_str(l.next().unwrap()).unwrap();
                DevicePair(sensor, beacon)
            })
            .collect();
        Ok(Self(pairs))
    }
}

impl DevicePairs {
    pub fn shape(&self) -> usize {
        (self.x_min() - self.x_max()).unsigned_abs()
    }

    // pub fn origin(&self) -> Point {}
    pub fn x_min(&self) -> isize {
        self.0.iter().map(|pair| pair.x_min()).min().unwrap()
    }
    pub fn x_max(&self) -> isize {
        self.0.iter().map(|pair| pair.x_max()).max().unwrap()
    }
}

pub struct Map {
    pub lane: Vec<Position>,
    pub devices: DevicePairs,
}
impl Map {
    pub fn new(s: &str, max_len: Option<usize>) -> Result<Self> {
        let devices = DevicePairs::from_str(s)?;
        let lane = if let Some(len) = max_len {
            vec![Position::default(); len]
        } else {
            vec![Position::default(); devices.shape()]
        };
        Ok(Self { lane, devices })
    }

    pub fn offset(&self) -> isize {
        self.devices.x_min()
    }
}
