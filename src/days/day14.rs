use std::{fmt::Display, str::FromStr};

use anyhow::{Error, Result};
use itertools::Itertools;
use std::iter::zip;

#[allow(dead_code)]
fn first() -> Result<String> {
    let ans = 0;
    let cave = Cave::new("src/days/data/day14_b.txt")?;
    println!("{}", cave.0);
    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let ans = 0;
    Ok(ans.to_string())
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        let x: usize = it.next().unwrap().parse()?;
        let y: usize = it.next().unwrap().parse()?;
        Ok(Self { x, y })
    }
}

#[derive(Debug)]
enum PointType {
    Air,
    Rock,
    Sand,
    SandSource,
}
impl Default for PointType {
    fn default() -> Self {
        Self::Air
    }
}
impl Display for PointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Air => ".",
            Self::Rock => "#",
            Self::Sand => "o",
            Self::SandSource => "+",
        };

        write!(f, "{s}")
    }
}
struct Cave(ndarray::Array2<PointType>);
impl Cave {
    pub fn new(file: &str) -> Result<Self> {
        let point_chains = Self::load_file(file)?;
        let points = point_chains.iter().flatten().collect_vec();
        let ((x_min, y_min), (x_max, y_max)) = Self::find_limits(&points);

        let cave = Cave(ndarray::Array2::default((
            (y_max - y_min) + 1,
            (x_max - x_min) + 1,
        )));
        let cave = Self::add_sand_source(cave, x_min);
        let cave = Self::add_rock_walls(cave, &point_chains, x_min);
        Ok(cave)
    }
    fn load_file(file: &str) -> Result<Vec<Vec<Point>>> {
        let data = std::fs::read_to_string(file)?;
        data.lines()
            .map(|line| {
                let raw_points = line.split(" -> ");
                raw_points.map(Point::from_str).collect()
            })
            .collect()
    }
    fn find_limits(points: &[&Point]) -> ((usize, usize), (usize, usize)) {
        let x_min = points.iter().min_by_key(|point| point.x).unwrap().x;
        let x_max = points.iter().max_by_key(|point| point.x).unwrap().x;
        let y_min = 0;
        let y_max = points.iter().max_by_key(|point| point.y).unwrap().y;
        ((x_min, y_min), (x_max, y_max))
    }
    fn add_sand_source(mut cave: Self, x_min: usize) -> Self {
        let source_point = Point {
            x: 500 - x_min,
            y: 0,
        };
        cave.0[[source_point.y, source_point.x]] = PointType::SandSource;
        cave
    }
    fn add_rock_walls(mut cave: Self, point_chains: &Vec<Vec<Point>>, x_min: usize) -> Self {
        let adjusted_chains = point_chains
            .into_iter()
            .map(|chain| {
                chain
                    .into_iter()
                    .map(|point| Point {
                        x: point.x - x_min,
                        y: point.y,
                    })
                    .collect_vec()
            })
            .collect_vec();
        for chain in adjusted_chains.iter() {
            for (point1, point2) in zip(
                chain.iter().take(chain.len() - 1),
                chain.iter().skip(1).take(chain.len() - 1),
            ) {
                let x_span = num::abs(point1.x as isize - point2.x as isize) as usize;
                let y_span = num::abs(point1.y as isize - point2.y as isize) as usize;
                let x_start = point1.x.min(point2.x);
                let y_start = point1.y.min(point2.y);
                let x_end = x_start + x_span;
                let y_end = y_start + y_span;
                for y in y_start..=y_end {
                    for x in x_start..=x_end {
                        cave.0[[y, x]] = PointType::Rock;
                    }
                }
            }
        }
        cave
    }
}

// (4,4), (4,5), (4,6)
// (4,2), (4,3), (4,4), (4,5), (4,6)
// (5,2), (5,3), (5,4), (5,5), (5,6)
// (6,2), (6,3), (6,4), (6,5), (6,6)

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
