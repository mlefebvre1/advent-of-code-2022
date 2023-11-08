use std::fmt::Display;

use ndarray::{Array2, ArrayView};

use crate::model::{
    action::JetPattern,
    rock::{
        Position, Rock, RockShape, Shape, ShapeHorLine, ShapePlus, ShapeRevL, ShapeSquare,
        ShapeVertLine, SHAPES,
    },
};

#[derive(Clone, Copy, Debug)]
pub enum Elements {
    FallingRock,
    StoppedRock,
    Empty,
    HorizontalWall,
    VerticalWall,
    CornerWall,
}

pub const EMPTY_LINE: [Elements; 9] = [
    Elements::VerticalWall,
    Elements::Empty,
    Elements::Empty,
    Elements::Empty,
    Elements::Empty,
    Elements::Empty,
    Elements::Empty,
    Elements::Empty,
    Elements::VerticalWall,
];
pub const FLOOR: [Elements; 9] = [
    Elements::CornerWall,
    Elements::HorizontalWall,
    Elements::HorizontalWall,
    Elements::HorizontalWall,
    Elements::HorizontalWall,
    Elements::HorizontalWall,
    Elements::HorizontalWall,
    Elements::HorizontalWall,
    Elements::CornerWall,
];

impl Display for Elements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::FallingRock => "@",
            Self::StoppedRock => "#",
            Self::Empty => ".",
            Self::HorizontalWall => "-",
            Self::VerticalWall => "|",
            Self::CornerWall => "+",
        };
        write!(f, "{s}")
    }
}

pub struct Cave {
    inner: Array2<Elements>,
    patterns: Box<dyn Iterator<Item = JetPattern>>,
}

impl Cave {
    pub fn new(patterns: Vec<JetPattern>) -> Self {
        let patterns = Box::new(patterns.into_iter().cycle());
        let inner = Array2::from_shape_vec((1, FLOOR.len()), FLOOR.to_vec()).unwrap();
        Self { inner, patterns }
    }

    pub fn run(&mut self, nb_rocks: usize) -> usize {
        for shape in SHAPES.into_iter().cycle().take(nb_rocks) {
            let rock = self.new_rock(shape);
            self.grow(&rock);
            self.run_fall(rock);
        }
        self.tower_height()
    }

    fn grow(&mut self, rock: &Rock) {
        let cave_height = self.inner.nrows();
        let tower_height = self.tower_height();
        let needed_height = tower_height + 3;

        let nb_new_empty_line =
            (needed_height as isize - cave_height as isize) + rock.shape.height() as isize;
        if nb_new_empty_line > 0 {
            for _ in 0..nb_new_empty_line {
                self.add_empty_line();
            }
        }
    }

    fn tower_height(&self) -> usize {
        for (i, row) in self.inner.rows().into_iter().enumerate() {
            if row.iter().all(|e| {
                !matches!(
                    e,
                    Elements::FallingRock | Elements::StoppedRock | Elements::HorizontalWall
                )
            }) {
                return i;
            }
        }
        self.inner.nrows()
    }

    fn add_empty_line(&mut self) {
        self.inner.push_row(ArrayView::from(&EMPTY_LINE)).unwrap();
    }

    fn new_rock(&mut self, shape: Shape) -> Rock {
        // all shapes starts at x=3!
        let pos = Position::new(3, self.tower_height() + 3);
        Rock { shape, pos }
    }

    fn run_fall(&mut self, mut rock: Rock) {
        // draw rock to the simulation
        self.draw(&rock, Elements::FallingRock);

        loop {
            let pattern = self.patterns.next().unwrap();
            self.apply_pattern(&mut rock, pattern);
            if self.fall(&mut rock) {
                return;
            }
        }
    }

    fn apply_pattern(&mut self, rock: &mut Rock, pattern: JetPattern) {
        let new_rock_pos = Position::new(
            pattern.apply(rock.pos.loc().0 as isize) as usize,
            rock.pos.loc().1,
        );
        let new_rock = Rock {
            shape: rock.shape,
            pos: new_rock_pos,
        };
        if self.valid_move(&new_rock) {
            self.mv(rock, new_rock_pos);
        }
    }

    fn fall(&mut self, rock: &mut Rock) -> bool {
        let new_rock_pos = Position::new(rock.pos.loc().0, rock.pos.loc().1 - 1);
        let new_rock = Rock {
            shape: rock.shape,
            pos: new_rock_pos,
        };
        if self.valid_move(&new_rock) {
            self.mv(rock, new_rock_pos);
            false
        } else {
            self.draw(rock, Elements::StoppedRock);
            true
        }
    }

    fn valid_move(&self, new_rock: &Rock) -> bool {
        new_rock.shape.loc().into_iter().all(|rel_pos| {
            let (x, y) = (new_rock.pos + rel_pos).loc();
            matches!(self.inner[[y, x]], Elements::Empty | Elements::FallingRock)
        })
    }

    fn mv(&mut self, rock: &mut Rock, new_pos: Position) {
        //erase rock
        self.draw(rock, Elements::Empty);
        //adjust pos
        rock.pos = new_pos;
        //draw rock
        self.draw(rock, Elements::FallingRock);
    }

    fn draw(&mut self, rock: &Rock, element: Elements) {
        for rel_pos in rock.shape.loc() {
            let (x, y) = (rock.pos + rel_pos).loc();
            self.inner[[y, x]] = element;
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for row in self.inner.axis_iter(ndarray::Axis(0)).rev() {
            for col in row {
                print!("{col}");
            }
            println!()
        }
    }
}
