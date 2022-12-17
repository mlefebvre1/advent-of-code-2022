use std::{collections::HashSet, hash::Hash, str::FromStr};

use anyhow::{anyhow, Error, Ok, Result};

#[allow(dead_code)]
fn first() -> Result<String> {
    let data = std::fs::read_to_string("src/days/data/day9.txt")?;
    let moves: Result<Vec<Move>> = data.lines().map(Move::from_str).collect();
    let mut game = RopeGame::new(2);
    game.run(moves?);
    Ok(game.tail_position_log.len().to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let data = std::fs::read_to_string("src/days/data/day9.txt")?;
    let moves: Result<Vec<Move>> = data.lines().map(Move::from_str).collect();
    let mut game = RopeGame::new(10);
    game.run(moves?);
    Ok(game.tail_position_log.len().to_string())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => Err(anyhow!("{s} is an unexpected Direction")),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    nb_steps: u32,
}

impl FromStr for Move {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let direction = Direction::from_str(s.next().unwrap())?;
        let nb_steps = s.next().unwrap().parse()?;
        Ok(Self {
            direction,
            nb_steps,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Position {
    x: isize,
    y: isize,
}

struct RopeGame {
    knots: Vec<Position>,
    tail_position_log: HashSet<Position>,
}

impl RopeGame {
    fn new(nb_knots: usize) -> Self {
        Self {
            knots: vec![Position { x: 0, y: 0 }; nb_knots],
            tail_position_log: HashSet::new(),
        }
    }
    fn run(&mut self, moves: Vec<Move>) {
        for mv in moves {
            self.apply_move_and_log(mv)
        }
    }
    fn apply_move_and_log(&mut self, mv: Move) {
        for _step in 0..mv.nb_steps {
            match mv.direction {
                Direction::Up => self.move_head_up(),
                Direction::Down => self.move_head_down(),
                Direction::Right => self.move_head_right(),
                Direction::Left => self.move_head_left(),
            };
            self.adjust_knots();
            self.log_tail_position();
        }
    }
    fn move_head_up(&mut self) {
        self.knots[0].y += 1;
    }
    fn move_head_down(&mut self) {
        self.knots[0].y -= 1;
    }
    fn move_head_right(&mut self) {
        self.knots[0].x += 1;
    }
    fn move_head_left(&mut self) {
        self.knots[0].x -= 1;
    }
    fn adjust_knots(&mut self) {
        for i in 1..self.knots.len() {
            Self::adjust_knot(self.knots[i - 1], &mut self.knots[i]);
        }
    }
    fn adjust_knot(knot1: Position, knot2: &mut Position) {
        let (dx, dy) = (knot1.x - knot2.x, knot1.y - knot2.y);
        if (dx > 1 && dy >= 1) || (dx >= 1 && dy > 1) {
            // Up-Right
            knot2.x += 1;
            knot2.y += 1;
        } else if (dx > 1 && dy <= -1) || (dx >= 1 && dy < -1) {
            // Down-Right
            knot2.x += 1;
            knot2.y -= 1;
        } else if (dx < -1 && dy >= 1) || (dx <= -1 && dy > 1) {
            // Left-Up
            knot2.x -= 1;
            knot2.y += 1;
        } else if (dx < -1 && dy <= -1) || (dx <= -1 && dy < -1) {
            // Left-Down
            knot2.x -= 1;
            knot2.y -= 1;
        } else if dx > 1 {
            // Right
            knot2.x += 1;
        } else if dx < -1 {
            // Left
            knot2.x -= 1;
        } else if dy > 1 {
            // Up
            knot2.y += 1;
        } else if dy < -1 {
            // Down
            knot2.y -= 1;
        }
    }
    fn log_tail_position(&mut self) {
        let tail = self.knots.last().unwrap();
        self.tail_position_log.insert(*tail);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_part() {
        println!("Day9 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_second_part() {
        println!("Day9 - Second problem : {}", second().unwrap())
    }
}
