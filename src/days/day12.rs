use crate::graph::{
    dgraph::{Dgraph, Edge},
    shortest_path::ShortestPath,
};
use anyhow::Result;
use itertools::Itertools;

#[allow(dead_code)]
fn first() -> Result<String> {
    let grid = make_grid("src/days/data/day12.txt")?;
    let starting_position = get_starting_positions(&grid)
        .into_iter()
        .find(|position| grid[position.y][position.x] == 'S')
        .unwrap();
    let end = get_end_position(&grid);
    let graph = build_graph(&grid);
    let ans = graph.disjktra_shortest_path(
        get_vertex_id_from_position(starting_position, &grid),
        get_vertex_id_from_position(end, &grid),
    );
    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let grid = make_grid("src/days/data/day12.txt")?;
    let starting_position = get_starting_positions(&grid);
    let end = get_end_position(&grid);
    let graph = build_graph(&grid);
    let ans = starting_position
        .iter()
        .map(|start| {
            graph.disjktra_shortest_path(
                get_vertex_id_from_position(*start, &grid),
                get_vertex_id_from_position(end, &grid),
            )
        })
        .min()
        .unwrap();
    Ok(ans.to_string())
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}
type Grid = Vec<Vec<char>>;

fn build_graph(grid: &Grid) -> Dgraph {
    let nb_vertices = grid.len() * grid[0].len();
    let mut graph = Dgraph::new(nb_vertices);
    connect_edges(&mut graph, grid);
    graph
}

fn make_grid(file_path: &str) -> Result<Grid> {
    let data = std::fs::read_to_string(file_path)?;
    let grid = data
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    Ok(grid)
}
fn get_starting_positions(grid: &Grid) -> Vec<Position> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, col)| {
                if matches!(col, 'a' | 'S') {
                    Some(Position { x, y })
                } else {
                    None
                }
            })
        })
        .collect_vec()
}
fn get_end_position(grid: &Grid) -> Position {
    let mut end = Position { x: 0, y: 0 };
    for (y, line) in grid.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if *col == 'E' {
                end = Position { x, y };
            }
        }
    }
    end
}
fn connect_edges(graph: &mut Dgraph, grid: &Grid) {
    for (y, line) in grid.iter().enumerate() {
        for (x, _ch) in line.iter().enumerate() {
            let current_position = Position { x, y };
            if let Some(next_position) = next_position_up(grid, current_position) {
                graph.add_edge(make_edge(current_position, next_position, grid));
            }
            if let Some(next_position) = next_position_down(grid, current_position) {
                graph.add_edge(make_edge(current_position, next_position, grid));
            }
            if let Some(next_position) = next_position_right(grid, current_position) {
                graph.add_edge(make_edge(current_position, next_position, grid));
            }
            if let Some(next_position) = next_position_left(grid, current_position) {
                graph.add_edge(make_edge(current_position, next_position, grid));
            }
        }
    }
}
fn make_edge(current_position: Position, next_position: Position, grid: &Grid) -> Edge {
    Edge {
        src: get_vertex_id_from_position(current_position, grid),
        dst: get_vertex_id_from_position(next_position, grid),
        weight: 1,
    }
}
fn get_vertex_id_from_position(position: Position, grid: &Grid) -> usize {
    let x_max = grid[0].len();
    position.y * x_max + position.x
}

fn next_position_up(grid: &Grid, current_position: Position) -> Option<Position> {
    let (next_x, next_y) = (current_position.x, current_position.y.checked_sub(1));
    next_y.and_then(|next_y| {
        let next_position = Position {
            x: next_x,
            y: next_y,
        };
        get_next_position(current_position, next_position, grid)
    })
}
fn next_position_down(grid: &Grid, current_position: Position) -> Option<Position> {
    let (next_x, next_y) = (current_position.x, current_position.y + 1);
    if current_position.y < grid.len() - 1 {
        let next_position = Position {
            x: next_x,
            y: next_y,
        };
        get_next_position(current_position, next_position, grid)
    } else {
        None
    }
}
fn next_position_right(grid: &Grid, current_position: Position) -> Option<Position> {
    let (next_x, next_y) = (current_position.x + 1, current_position.y);
    if current_position.x < grid[0].len() - 1 {
        let next_position = Position {
            x: next_x,
            y: next_y,
        };
        get_next_position(current_position, next_position, grid)
    } else {
        None
    }
}
fn next_position_left(grid: &Grid, current_position: Position) -> Option<Position> {
    let (next_x, next_y) = (current_position.x.checked_sub(1), current_position.y);
    next_x.and_then(|next_x| {
        let next_position = Position {
            x: next_x,
            y: next_y,
        };
        get_next_position(current_position, next_position, grid)
    })
}
fn get_next_position(
    current_position: Position,
    next_position: Position,
    grid: &Grid,
) -> Option<Position> {
    let current_ch = grid[current_position.y][current_position.x];
    let next_ch = grid[next_position.y][next_position.x];
    if is_next_char_valid(current_ch, next_ch) {
        Some(next_position)
    } else {
        None
    }
}
fn is_next_char_valid(current_ch: char, next_ch: char) -> bool {
    match next_ch {
        'S' => false,
        'E' => current_ch == 'z',
        _ => match current_ch {
            'S' => next_ch == 'a',
            _ => current_ch as u8 + 1 >= next_ch as u8,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_part() {
        println!("Day12 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_second_part() {
        println!("Day12 - Second problem : {}", second().unwrap())
    }
}
