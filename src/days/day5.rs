use anyhow::Result;
use itertools::Itertools;
use std::fs;

#[allow(dead_code)]
fn first() -> Result<String> {
    let data = fs::read_to_string("src/days/data/day5.txt")?;
    let mut lines = data.lines();
    let lines = lines.by_ref();

    let lines_for_stacks: Vec<&str> = lines.take_while(|line| !line.is_empty()).collect();
    let stacks = create_stacks(&lines_for_stacks);

    let lines_for_moves: Vec<&str> = lines.collect();
    let moves = create_moves(&lines_for_moves);

    let final_stacks = apply_stack_moves(stacks, moves);
    let ans = get_top_crates(final_stacks);
    Ok(ans)
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let data = fs::read_to_string("src/days/data/day5.txt")?;
    let mut lines = data.lines();
    let lines = lines.by_ref();

    let lines_for_stacks: Vec<&str> = lines.take_while(|line| !line.is_empty()).collect();
    let stacks = create_stacks(&lines_for_stacks);

    let lines_for_moves: Vec<&str> = lines.collect();
    let moves = create_moves(&lines_for_moves);

    let final_stacks = apply_stack_moves_9001(stacks, moves);
    let ans = get_top_crates(final_stacks);
    Ok(ans)
}
#[derive(Debug)]
struct Move {
    nb_to_pop: usize,
    stack_to_pop: usize,
    stack_to_push: usize,
}

fn create_stacks(lines: &[&str]) -> Vec<Vec<char>> {
    let line_with_num = lines.last().unwrap();
    let nb_stacks = line_with_num
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).unwrap())
        .max()
        .unwrap();
    let mut stacks = vec![vec![]; nb_stacks as usize];
    for line in lines.iter().rev().skip(1) {
        for (c, n) in line.chars().zip(line_with_num.chars()) {
            if !n.is_whitespace() && !c.is_whitespace() {
                stacks[(n.to_digit(10).unwrap() - 1) as usize].push(c);
            }
        }
    }
    stacks
}

fn create_moves(lines: &[&str]) -> Vec<Move> {
    lines
        .iter()
        .map(|line| {
            let mut line = line.chars();
            let line = line.by_ref();
            let _ = line.take(5).last(); // remove "move "
            let nb_to_pop = get_number(line).unwrap();
            let _ = line.take(5).last(); // remove "from "
            let stack_to_pop = get_number(line).unwrap();
            let _ = line.take(3).last(); // remove "to "
            let stack_to_push = get_number(line).unwrap();
            Move {
                nb_to_pop,
                stack_to_pop,
                stack_to_push,
            }
        })
        .collect()
}

fn get_number(line: &mut impl Iterator<Item = char>) -> Result<usize> {
    let n: String = line.take_while(|c| !c.is_whitespace()).collect();
    Ok(n.parse::<usize>()?)
}

fn apply_stack_moves(mut stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    moves.iter().for_each(
        |Move {
             nb_to_pop,
             stack_to_pop,
             stack_to_push,
         }| {
            for _ in 0..*nb_to_pop {
                let crate_ = stacks[*stack_to_pop - 1].pop().unwrap();
                stacks[*stack_to_push - 1].push(crate_);
            }
        },
    );
    stacks
}

fn apply_stack_moves_9001(mut stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    moves.iter().for_each(
        |Move {
             nb_to_pop,
             stack_to_pop,
             stack_to_push,
         }| {
            let mut crates = (0..*nb_to_pop)
                .map(|_| stacks[*stack_to_pop - 1].pop().unwrap())
                .collect_vec();
            crates.reverse();
            stacks[*stack_to_push - 1].append(&mut crates);
        },
    );
    stacks
}

fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part1() {
        println!("Day5 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_part2() {
        println!("Day5 - Second problem : {}", second().unwrap())
    }
}
