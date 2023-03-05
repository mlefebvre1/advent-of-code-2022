use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Sensor(Point);
#[derive(Debug)]
pub struct Beacon(Point);

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
