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

#[derive(Clone)]
enum ValveIndex {
    Broken(usize),
    Nonbroken(usize),
}

pub struct Solve {
    distance_matrix: Array2<usize>,
    nodes: Vec<Valve>,
    valve_pressures: Vec<(usize, usize)>,
    best_pressure: RefCell<usize>,
}

#[derive(Clone)]
struct State {
    current_valve_index: ValveIndex,
    mins_remaining: isize,
}

impl Solve {
    pub fn new(data: &str) -> Self {
        let nodes = data
            .lines()
            .map(|line| Valve::from_str(line).unwrap())
            .collect::<Vec<Valve>>();
        let mut distance_matrix = Self::matrix_from_nodes(&nodes);
        aoc_utils::graph::shortest_path::floyd_warshal(&mut distance_matrix);
        let valve_pressures = nodes
            .iter()
            .enumerate()
            .filter(|(_i, valve)| valve.flow_rate > 0)
            .map(|(i, valve)| (i, valve.flow_rate))
            .collect_vec();
        Self {
            nodes,
            best_pressure: RefCell::new(0),
            distance_matrix,
            valve_pressures,
        }
    }

    pub fn solve_part1(&self) -> usize {
        let start_index = self
            .nodes
            .iter()
            .position(|valve| valve.label == "AA")
            .unwrap();
        let valves_visited = vec![false; self.valve_pressures.len()];
        self.solve_part1_recurse(ValveIndex::Broken(start_index), 0, 30, valves_visited);
        *self.best_pressure.borrow()
    }

    fn solve_part1_recurse(
        &self,
        current_index: ValveIndex,
        total_pressure: usize,
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
                let current_valve_absolute_index = match current_index {
                    ValveIndex::Broken(j) => j,
                    ValveIndex::Nonbroken(j) => self.valve_pressures[j].0,
                };
                let other_valve_absolute_index = self.valve_pressures[i].0;
                let new_mins = mins
                    - (self.distance_matrix
                        [[current_valve_absolute_index, other_valve_absolute_index]]
                        + 1) as isize;
                let new_pressure = if new_mins < 0 {
                    total_pressure
                } else {
                    total_pressure + (new_mins as usize * self.valve_pressures[i].1)
                };
                new_valve_visited[i] = true;
                self.solve_part1_recurse(
                    ValveIndex::Nonbroken(i),
                    new_pressure,
                    new_mins,
                    new_valve_visited,
                )
            }
        }
    }

    pub fn solve_part2(&self) -> usize {
        let start_index = self
            .nodes
            .iter()
            .position(|valve| valve.label == "AA")
            .unwrap();

        let valves_visited = vec![false; self.valve_pressures.len()];

        let human_state = State {
            current_valve_index: ValveIndex::Broken(start_index),
            mins_remaining: 26,
        };
        let elephant_state = State {
            current_valve_index: ValveIndex::Broken(start_index),
            mins_remaining: 26,
        };

        self.solve_part2_recurse(0, valves_visited, human_state, elephant_state);
        *self.best_pressure.borrow()
    }

    fn solve_part2_recurse(
        &self,
        total_pressure: usize,
        valves_visited: Vec<bool>,
        human_state: State,
        elephant_state: State,
    ) {
        if valves_visited.iter().all(|visited| *visited)
            || (human_state.mins_remaining <= 0 && elephant_state.mins_remaining <= 0)
        {
            let mut best_pressure = self.best_pressure.borrow_mut();
            if total_pressure > *best_pressure {
                *best_pressure = total_pressure;
            }
            return;
        }

        let unvisited_indexes = valves_visited
            .iter()
            .enumerate()
            .filter(|(_, &visited)| !visited)
            .map(|(i, _)| i)
            .collect_vec();
        let human_current_valve_absolute_index = match human_state.current_valve_index {
            ValveIndex::Broken(j) => j,
            ValveIndex::Nonbroken(j) => self.valve_pressures[j].0,
        };
        let elephant_current_valve_absolute_index = match elephant_state.current_valve_index {
            ValveIndex::Broken(j) => j,
            ValveIndex::Nonbroken(j) => self.valve_pressures[j].0,
        };
        if unvisited_indexes.len() > 1 {
            for pair in unvisited_indexes.iter().permutations(2) {
                let mut new_valve_visited = valves_visited.clone();
                let mut new_total_pressure = total_pressure;
                let mut pair_it = pair.iter();
                let next_human_index = **pair_it.by_ref().next().unwrap();
                let human_other_valve_absolute_index = self.valve_pressures[next_human_index].0;
                let human_new_mins = human_state.mins_remaining
                    - (self.distance_matrix[[
                        human_current_valve_absolute_index,
                        human_other_valve_absolute_index,
                    ]] + 1) as isize;
                let new_human_total_pressure = Self::get_new_total_pressure(
                    human_new_mins,
                    self.valve_pressures[next_human_index].1,
                );

                let next_elephant_index = **pair_it.next().unwrap();
                let elephant_other_valve_absolute_index =
                    self.valve_pressures[next_elephant_index].0;
                let elephant_new_mins = elephant_state.mins_remaining
                    - (self.distance_matrix[[
                        elephant_current_valve_absolute_index,
                        elephant_other_valve_absolute_index,
                    ]] + 1) as isize;
                let new_elephant_total_pressure = Self::get_new_total_pressure(
                    elephant_new_mins,
                    self.valve_pressures[next_elephant_index].1,
                );
                let (new_human_state, new_elephant_state) = Self::update_state(
                    &human_state,
                    &elephant_state,
                    &mut new_valve_visited,
                    next_human_index,
                    human_new_mins,
                    new_human_total_pressure,
                    next_elephant_index,
                    new_elephant_total_pressure,
                    elephant_new_mins,
                    &mut new_total_pressure,
                    false,
                );
                self.solve_part2_recurse(
                    new_total_pressure,
                    new_valve_visited,
                    new_human_state,
                    new_elephant_state,
                )
            }
        } else {
            let mut new_valve_visited = valves_visited.clone();
            let mut new_total_pressure = total_pressure;

            let next_i = *unvisited_indexes.iter().next().unwrap();
            let human_other_valve_absolute_index = self.valve_pressures[next_i].0;
            let human_new_mins = human_state.mins_remaining
                - (self.distance_matrix[[
                    human_current_valve_absolute_index,
                    human_other_valve_absolute_index,
                ]] + 1) as isize;
            let new_human_total_pressure =
                Self::get_new_total_pressure(human_new_mins, self.valve_pressures[next_i].1);

            let elephant_other_valve_absolute_index = self.valve_pressures[next_i].0;
            let elephant_new_mins = elephant_state.mins_remaining
                - (self.distance_matrix[[
                    elephant_current_valve_absolute_index,
                    elephant_other_valve_absolute_index,
                ]] + 1) as isize;
            let new_elephant_total_pressure =
                Self::get_new_total_pressure(elephant_new_mins, self.valve_pressures[next_i].1);

            let (new_human_state, new_elephant_state) = Self::update_state(
                &human_state,
                &elephant_state,
                &mut new_valve_visited,
                next_i,
                human_new_mins,
                new_human_total_pressure,
                next_i,
                new_elephant_total_pressure,
                elephant_new_mins,
                &mut new_total_pressure,
                true,
            );
            self.solve_part2_recurse(
                new_total_pressure,
                new_valve_visited,
                new_human_state,
                new_elephant_state,
            )
        }
    }

    #[inline]
    fn update_state(
        human_state: &State,
        elephant_state: &State,
        new_valve_visited: &mut [bool],
        new_human_index: usize,
        new_human_mins: isize,
        new_human_total_pressure: usize,
        new_elephant_index: usize,
        new_elephant_total_pressure: usize,
        new_elephant_mins: isize,
        total_pressure: &mut usize,
        one: bool,
    ) -> (State, State) {
        let mut new_human_state = human_state.clone();
        let mut new_elephant_state = elephant_state.clone();

        if one {
            if new_human_state.mins_remaining > new_elephant_state.mins_remaining {
                new_human_state.mins_remaining = new_human_mins;
                new_human_state.current_valve_index = ValveIndex::Nonbroken(new_human_index);
                new_valve_visited[new_human_index] = true;
                *total_pressure += new_human_total_pressure;
            } else {
                new_elephant_state.mins_remaining = new_elephant_mins;
                new_elephant_state.current_valve_index = ValveIndex::Nonbroken(new_elephant_index);
                new_valve_visited[new_elephant_index] = true;
                *total_pressure += new_elephant_total_pressure;
            }
        } else {
            new_human_state.mins_remaining = new_human_mins;
            new_human_state.current_valve_index = ValveIndex::Nonbroken(new_human_index);
            new_valve_visited[new_human_index] = true;
            *total_pressure += new_human_total_pressure;

            new_elephant_state.mins_remaining = new_elephant_mins;
            new_elephant_state.current_valve_index = ValveIndex::Nonbroken(new_elephant_index);
            new_valve_visited[new_elephant_index] = true;
            *total_pressure += new_elephant_total_pressure;
        }

        (new_human_state, new_elephant_state)
    }

    #[inline]
    fn get_new_total_pressure(mins_remaining: isize, flow_rate: usize) -> usize {
        if mins_remaining < 0 {
            0
        } else {
            mins_remaining as usize * flow_rate
        }
    }

    fn matrix_from_nodes(nodes: &[Valve]) -> Array2<usize> {
        let n = nodes.len();
        let mut mat = ndarray::Array2::<usize>::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    mat[[i, j]] = 0;
                } else if i != j && nodes[i].neighbors.contains(&nodes[j].label) {
                    // check if a path exists from node i to node node j
                    mat[[i, j]] = 1;
                } else {
                    mat[[i, j]] = usize::MAX;
                }
            }
        }
        mat
    }
}
