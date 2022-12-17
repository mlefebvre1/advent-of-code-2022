use anyhow::Result;
use itertools::Itertools;
use num::integer::Roots;

#[allow(dead_code)]
fn first() -> Result<String> {
    let grid = create_grid()?;
    let visible_grid = get_visible_trees(&grid);
    let ans = visible_grid.iter().filter(|&visible| *visible).count();
    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let grid = create_grid()?;
    let tree_scenic_score = get_tree_scenic_score(&grid);
    let ans = tree_scenic_score.iter().max().unwrap();
    Ok(ans.to_string())
}

type Grid = ndarray::Array2<u32>;
type VisibleGrid = ndarray::Array2<bool>;

fn create_grid() -> Result<Grid> {
    let data = std::fs::read_to_string("src/days/data/day8.txt")?;
    let grid = data
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let dim = grid.len().sqrt();
    let grid = ndarray::Array2::from_shape_vec((dim, dim), grid)?;
    Ok(grid)
}

fn get_visible_trees(grid: &Grid) -> VisibleGrid {
    let (xmax, ymax) = grid.dim();
    let mut visible_grid = ndarray::Array2::<bool>::from_shape_fn((xmax, ymax), |_| false);
    for y in 0..ymax {
        for x in 0..xmax {
            if is_highest_up(grid, x, y)
                || is_highest_down(grid, x, y)
                || is_highest_right(grid, x, y)
                || is_highest_left(grid, x, y)
            {
                visible_grid[[y, x]] = true;
            }
        }
    }
    visible_grid
}
fn is_highest_up(grid: &Grid, x: usize, y: usize) -> bool {
    let tree_height = grid[[y, x]];
    !grid
        .column(x)
        .iter()
        .take(y)
        .any(|&other_height| other_height >= tree_height)
}
fn is_highest_down(grid: &Grid, x: usize, y: usize) -> bool {
    let tree_height = grid[[y, x]];
    !grid
        .column(x)
        .iter()
        .skip(y + 1)
        .any(|&other_height| other_height >= tree_height)
}
fn is_highest_left(grid: &Grid, x: usize, y: usize) -> bool {
    let tree_height = grid[[y, x]];
    !grid
        .row(y)
        .iter()
        .take(x)
        .any(|&other_height| other_height >= tree_height)
}
fn is_highest_right(grid: &Grid, x: usize, y: usize) -> bool {
    let tree_height = grid[[y, x]];
    !grid
        .row(y)
        .iter()
        .skip(x + 1)
        .any(|&other_height| other_height >= tree_height)
}

fn get_tree_scenic_score(grid: &Grid) -> Grid {
    let (xmax, ymax) = grid.dim();
    let mut scenic_score = ndarray::Array2::<u32>::ones((xmax, ymax));
    for y in 1..ymax - 1 {
        for x in 1..xmax - 1 {
            scenic_score[[y, x]] *= scenic_score_up(grid, x, y);
            scenic_score[[y, x]] *= scenic_score_down(grid, x, y);
            scenic_score[[y, x]] *= scenic_score_left(grid, x, y);
            scenic_score[[y, x]] *= scenic_score_right(grid, x, y);
        }
    }
    scenic_score
}

fn scenic_score_up(grid: &Grid, x: usize, y: usize) -> u32 {
    let house_height = grid[[y, x]];
    let mut score = 0;
    for tree_height in grid.column(x).iter().take(y).rev() {
        score += 1;
        if *tree_height >= house_height {
            break;
        }
    }
    score
}
fn scenic_score_down(grid: &Grid, x: usize, y: usize) -> u32 {
    let house_height = grid[[y, x]];
    let mut score = 0;
    for tree_height in grid.column(x).iter().skip(y + 1) {
        score += 1;
        if *tree_height >= house_height {
            break;
        }
    }
    score
}
fn scenic_score_left(grid: &Grid, x: usize, y: usize) -> u32 {
    let house_height = grid[[y, x]];
    let mut score = 0;
    for tree_height in grid.row(y).iter().take(x).rev() {
        score += 1;
        if *tree_height >= house_height {
            break;
        }
    }
    score
}
fn scenic_score_right(grid: &Grid, x: usize, y: usize) -> u32 {
    let house_height = grid[[y, x]];
    let mut score = 0;
    for tree_height in grid.row(y).iter().skip(x + 1) {
        score += 1;
        if *tree_height >= house_height {
            break;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part1() {
        println!("Day8 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_part2() {
        println!("Day8 - Second problem : {}", second().unwrap())
    }
}
