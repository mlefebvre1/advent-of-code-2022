use std::{fmt::Display, str::FromStr};

use anyhow::{Error, Result};
use itertools::Itertools;
use std::iter::zip;

fn main() -> Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

fn first() -> Result<String> {
    let mut cave = Cave::new_part1("day14/data/day14.txt")?;
    cave.run();
    let ans = cave.nb_sands();
    Ok(ans.to_string())
}

fn second() -> Result<String> {
    let mut cave = Cave::new_part2("day14/data/day14.txt")?;
    cave.run();
    let ans = cave.nb_sands();
    Ok(ans.to_string())
}

#[derive(Debug, Clone, Copy)]
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
struct Cave {
    grid: ndarray::Array2<PointType>,
    sand_source: Point,
    xmax: usize,
    ymax: usize,
}
impl Cave {
    pub fn new_part1(file: &str) -> Result<Self> {
        let point_chains = Self::load_file(file)?;
        let points = point_chains.iter().flatten().collect_vec();
        let ((x_min, y_min), (x_max, y_max)) = Self::find_limits(&points);
        let (dy, dx) = (y_max - y_min, x_max - x_min);
        let cave = Self {
            grid: ndarray::Array2::default((dy + 1, dx + 1)),
            sand_source: Point {
                x: 500 - x_min,
                y: 0,
            },
            xmax: dx,
            ymax: dy,
        };
        let cave = Self::add_sand_source(cave);
        let cave = Self::add_rock_walls(cave, &point_chains, x_min);
        Ok(cave)
    }
    pub fn new_part2(file: &str) -> Result<Self> {
        let point_chains = Self::load_file(file)?;
        let points = point_chains.iter().flatten().collect_vec();
        let ((x_min, y_min), (x_max, y_max)) = Self::find_limits(&points);
        let (dy, dx) = (y_max - y_min, x_max - x_min);
        // x space needs to be much larger because sand will exceed x_min and x_max (look at the example they provide)
        let cave = Self {
            grid: ndarray::Array2::default(((dy + 2) + 1, (dx * 8) + 1)),
            sand_source: Point { x: dx * 4, y: 0 },
            xmax: dx * 8,
            ymax: dy + 2,
        };
        let cave = Self::add_sand_source(cave);
        let cave = Self::add_rock_walls(cave, &point_chains, 500 - dx * 4);
        let cave = Self::add_floor(cave, y_max);
        Ok(cave)
    }
    pub fn run(&mut self) {
        while let Some(sand) = self.get_next_sand() {
            self.grid[[sand.y, sand.x]] = PointType::Sand;
            if sand.x == self.sand_source.x && sand.y == self.sand_source.y {
                break;
            }
        }
    }
    pub fn nb_sands(&self) -> usize {
        self.grid
            .iter()
            .filter(|point_type| matches!(point_type, PointType::Sand))
            .count()
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
    fn add_sand_source(mut cave: Self) -> Self {
        cave.grid[[cave.sand_source.y, cave.sand_source.x]] = PointType::SandSource;
        cave
    }
    fn add_rock_walls(mut cave: Self, point_chains: &[Vec<Point>], x_translation: usize) -> Self {
        let adjusted_chains = point_chains
            .iter()
            .map(|chain| {
                chain
                    .iter()
                    .map(|point| Point {
                        x: point.x - x_translation,
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
                        cave.grid[[y, x]] = PointType::Rock;
                    }
                }
            }
        }
        cave
    }
    fn add_floor(mut cave: Self, ymax: usize) -> Self {
        cave.grid
            .row_mut(ymax + 2)
            .into_iter()
            .for_each(|p| *p = PointType::Rock);
        cave
    }

    fn get_next_sand(&self) -> Option<Point> {
        let mut sand_position = self.sand_source;
        loop {
            if !self.is_sand_blocked_down(sand_position) {
                if sand_position.y + 1 == self.ymax {
                    return None;
                } else {
                    sand_position.y += 1;
                    continue;
                }
            }
            if !self.is_sand_blocked_down_left(sand_position) {
                if sand_position.x - 1 == 0 || sand_position.y + 1 == self.ymax {
                    return None;
                } else {
                    sand_position.x -= 1;
                    sand_position.y += 1;
                    continue;
                }
            }
            if !self.is_sand_blocked_down_right(sand_position) {
                if sand_position.x + 1 == self.xmax || sand_position.y + 1 == self.ymax {
                    return None;
                } else {
                    sand_position.x += 1;
                    sand_position.y += 1;
                    continue;
                }
            }
            break;
        }
        Some(sand_position)
    }

    fn is_sand_blocked_down(&self, sand_position: Point) -> bool {
        let (x, y) = (sand_position.x, sand_position.y + 1);
        matches!(self.grid[[y, x]], PointType::Rock | PointType::Sand)
    }

    fn is_sand_blocked_down_left(&self, sand_position: Point) -> bool {
        let (x, y) = (sand_position.x - 1, sand_position.y + 1);
        matches!(self.grid[[y, x]], PointType::Rock | PointType::Sand)
    }

    fn is_sand_blocked_down_right(&self, sand_position: Point) -> bool {
        let (x, y) = (sand_position.x + 1, sand_position.y + 1);
        matches!(self.grid[[y, x]], PointType::Rock | PointType::Sand)
    }
}
