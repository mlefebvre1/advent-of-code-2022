use anyhow::Result;
use itertools::Itertools;
use num::integer::Roots;

#[allow(dead_code)]
fn first() -> Result<String> {
    /*
    --- Day 8: Treetop Tree House ---

    The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted
    these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

    First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are
    visible from outside the grid when looking directly along a row or column.

    The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

    30373
    25512
    65332
    33549
    35390
    Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

    A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column;
    that is, only look up, down, left, or right from any given tree.

    All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view.
    In this example, that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
    With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

    Consider your map; how many trees are visible from outside the grid?
    */
    let grid = create_grid()?;
    let visible_grid = get_visible_trees(&grid);
    let ans = visible_grid.iter().filter(|&visible| *visible).count();
    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    /*
    -- Part Two ---

    Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able
    to see a lot of trees.

    To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree
    that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances
    will be zero.)

    The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry,
    so they wouldn't be able to see higher than the tree house anyway.

    In the example above, consider the middle 5 in the second row:

    30373
    25512
    65332
    33549
    35390
    Looking up, its view is not blocked; it can see 1 tree (of height 3).
    Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
    Looking right, its view is not blocked; it can see 2 trees.
    Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).
    A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4
    (found by multiplying 1 * 1 * 2 * 2).

    However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

    30373
    25512
    65332
    33549
    35390
    Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
    Looking left, its view is not blocked; it can see 2 trees.
    Looking down, its view is also not blocked; it can see 1 tree.
    Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
    This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

    Consider each tree on your map. What is the highest scenic score possible for any tree?
    */
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
    fn solve() {
        println!("Day8 - First problem : {}", first().unwrap());
        println!("Day8 - Second problem : {}", second().unwrap())
    }
}
