use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct Sensor(pub Point);
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
            Err(anyhow::anyhow!("Bad prefix when parsing for sensor"))
        }
    }
}
impl From<(isize, isize)> for Sensor {
    fn from(value: (isize, isize)) -> Self {
        Self(Point {
            x: value.0,
            y: value.1,
        })
    }
}
impl PartialEq for Sensor {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

#[derive(Debug, Clone)]
pub struct Beacon(pub Point);
impl FromStr for Beacon {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix(" closest beacon is at") {
            let mut coord = s.split(',');
            let x = get_coord_value(&mut coord)?;
            let y = get_coord_value(&mut coord)?;
            Ok(Self(Point { x, y }))
        } else {
            Err(anyhow::anyhow!("Bad prefix when parsing for beacon"))
        }
    }
}
impl From<(isize, isize)> for Beacon {
    fn from(value: (isize, isize)) -> Self {
        Self(Point {
            x: value.0,
            y: value.1,
        })
    }
}
impl PartialEq for Beacon {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
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
        .split('=')
        .nth(1)
        .unwrap()
        .parse()
}
