use std::ops::Add;

pub const SHAPES: [Shape; 5] = [
    Shape::HorLine(ShapeHorLine),
    Shape::Plus(ShapePlus),
    Shape::RevL(ShapeRevL),
    Shape::VertLine(ShapeVertLine),
    Shape::Square(ShapeSquare),
];

#[derive(Clone, Copy)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
    pub fn loc(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
#[derive(Clone)]
pub struct Rock {
    pub shape: Shape,
    pub pos: Position,
}

pub trait RockShape {
    fn loc(&self) -> Vec<Position>;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

/*
    ####
*/
#[derive(Clone, Copy, Debug)]
pub struct ShapeHorLine;
impl RockShape for ShapeHorLine {
    fn loc(&self) -> Vec<Position> {
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(3, 0),
        ]
    }
    fn height(&self) -> usize {
        1
    }
    fn width(&self) -> usize {
        4
    }
}

/*
   .#.
   ###
   .#.
*/
#[derive(Clone, Copy, Debug)]
pub struct ShapePlus;
impl RockShape for ShapePlus {
    fn loc(&self) -> Vec<Position> {
        vec![
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(1, 2),
        ]
    }
    fn height(&self) -> usize {
        3
    }
    fn width(&self) -> usize {
        3
    }
}

/*
   ..#
   ..#
   ###
*/
#[derive(Clone, Copy, Debug)]
pub struct ShapeRevL;
impl RockShape for ShapeRevL {
    fn loc(&self) -> Vec<Position> {
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(2, 2),
        ]
    }
    fn height(&self) -> usize {
        3
    }
    fn width(&self) -> usize {
        3
    }
}

/*
   #
   #
   #
   #
*/
#[derive(Clone, Copy, Debug)]
pub struct ShapeVertLine;
impl RockShape for ShapeVertLine {
    fn loc(&self) -> Vec<Position> {
        vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(0, 3),
        ]
    }
    fn height(&self) -> usize {
        4
    }
    fn width(&self) -> usize {
        1
    }
}

/*
   ##
   ##
*/
#[derive(Clone, Copy, Debug)]
pub struct ShapeSquare;
impl RockShape for ShapeSquare {
    fn loc(&self) -> Vec<Position> {
        vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 1),
        ]
    }
    fn height(&self) -> usize {
        2
    }
    fn width(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    HorLine(ShapeHorLine),
    Plus(ShapePlus),
    RevL(ShapeRevL),
    VertLine(ShapeVertLine),
    Square(ShapeSquare),
}

impl Default for Shape {
    fn default() -> Self {
        Self::HorLine(ShapeHorLine)
    }
}

impl RockShape for Shape {
    fn loc(&self) -> Vec<Position> {
        match self {
            Self::HorLine(hline) => hline.loc(),
            Self::Plus(plus) => plus.loc(),
            Self::RevL(revl) => revl.loc(),
            Self::Square(sq) => sq.loc(),
            Self::VertLine(vline) => vline.loc(),
        }
    }

    fn height(&self) -> usize {
        match self {
            Self::HorLine(hline) => hline.height(),
            Self::Plus(plus) => plus.height(),
            Self::RevL(revl) => revl.height(),
            Self::Square(sq) => sq.height(),
            Self::VertLine(vline) => vline.height(),
        }
    }

    fn width(&self) -> usize {
        match self {
            Self::HorLine(hline) => hline.height(),
            Self::Plus(plus) => plus.height(),
            Self::RevL(revl) => revl.height(),
            Self::Square(sq) => sq.height(),
            Self::VertLine(vline) => vline.height(),
        }
    }
}
