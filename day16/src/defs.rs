use itertools::Itertools;
use ndarray::Array2;
use std::{cell::RefCell, str::FromStr};

#[derive(Debug)]
pub struct Valve {
    pub label: String,
    pub flow_rate: usize,
    pub neighbors: Vec<String>,
}

impl FromStr for Valve {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        let s = s.by_ref();
        let label = s.skip(1).next().unwrap().to_string();
        let flow_rate = s
            .skip(2) // skip "has flow"
            .next()
            .unwrap()
            .split("=")
            .skip(1) // skip "rate"
            .next()
            .unwrap()
            .replace(";", "")
            .parse()?;

        let neighbors = s
            .skip(4)
            .map(|item| item.replace(",", ""))
            .collect::<Vec<_>>();
        Ok(Self {
            label,
            neighbors,
            flow_rate,
        })
    }
}

pub struct Nodes {
    nodes: Vec<Valve>,
    best_pressure: RefCell<usize>,
}

impl Nodes {
    pub fn new(data: &str) -> Self {
        let nodes = data
            .lines()
            .map(|line| Valve::from_str(line).unwrap())
            .collect::<Vec<Valve>>();
        Self {
            nodes,
            best_pressure: RefCell::new(0),
        }
    }

    pub fn solve(&self) {
        let mut distance_matrix = matrix_from_nodes(&self);
        aoc_utils::graph::shortest_path::floyd_warshal(&mut distance_matrix);
        println!("{distance_matrix}");

        let start_index = self
            .nodes
            .iter()
            .position(|valve| valve.label == "AA")
            .unwrap();

        let valve_pressures = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_i, valve)| valve.flow_rate > 0)
            .map(|(i, valve)| (i, valve.flow_rate))
            .collect_vec();

        let valves_visited = vec![false; valve_pressures.len()];
        self.solve_recurse(
            Some(start_index),
            start_index,
            0,
            &distance_matrix,
            &valve_pressures,
            30,
            valves_visited,
        );
        println!("{}", self.best_pressure.borrow());
    }

    fn solve_recurse(
        &self,
        first: Option<usize>,
        current_nonbroken_index: usize, //current non-broken valve relative index
        total_pressure: usize,
        dist_matrix: &Array2<usize>,
        valve_pressures: &[(usize, usize)],
        mins: isize,
        valves_visited: Vec<bool>,
    ) {
        if valves_visited.iter().all(|visited| *visited) || mins <= 0 {
            let mut best_pressure = self.best_pressure.borrow_mut();
            if total_pressure > *best_pressure {
                *best_pressure = total_pressure;
            }
            return;
        }
        for (i, visited) in valves_visited.iter().enumerate() {
            if !visited {
                let mut new_valve_visited = valves_visited.clone();
                let current_valve_absolute_index = if let Some(index) = first {
                    index
                } else {
                    valve_pressures[current_nonbroken_index].0
                };
                let other_valve_absolute_index = valve_pressures[i].0;
                let new_mins = mins
                    - (dist_matrix[[current_valve_absolute_index, other_valve_absolute_index]] + 1)
                        as isize;
                let new_pressure = if new_mins < 0 {
                    total_pressure
                } else {
                    total_pressure + (new_mins as usize * valve_pressures[i].1)
                };
                new_valve_visited[i] = true;
                self.solve_recurse(
                    None,
                    i,
                    new_pressure,
                    dist_matrix,
                    valve_pressures,
                    new_mins,
                    new_valve_visited,
                )
            }
        }
    }
}

pub fn matrix_from_nodes(nodes: &Nodes) -> Array2<usize> {
    let n = nodes.nodes.len();
    let mut mat = ndarray::Array2::<usize>::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            if i == j {
                mat[[i, j]] = 0;
            } else if i != j && nodes.nodes[i].neighbors.contains(&nodes.nodes[j].label) {
                // check if a path exists from node i to node node j
                mat[[i, j]] = 1;
            } else {
                mat[[i, j]] = usize::MAX;
            }
        }
    }
    mat
}
