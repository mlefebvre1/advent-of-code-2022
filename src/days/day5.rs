use anyhow::Result;
use itertools::Itertools;
use std::fs;

#[allow(dead_code)]
fn first() -> Result<String> {
    /*
    --- Day 5: Supply Stacks ---
    The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates,
    but because the needed supplies are buried under many other crates, the crates need to be rearranged.

    The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane
    operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top
    of each stack.

    The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where,
    and they want to be ready to unload them as soon as possible so they can embark.

    They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains
    three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

    Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack.
    In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

    [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3
    In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up
    below the second and third crates:

            [Z]
            [N]
        [C] [D]
        [M] [P]
     1   2   3
    Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

            [Z]
            [N]
    [M]     [D]
    [C]     [P]
     1   2   3
    Finally, one crate is moved from stack 1 to stack 2:

            [Z]
            [N]
            [D]
    [C] [M] [P]
     1   2   3
    The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z
    in stack 3, so you should combine these together and give the Elves the message CMZ.

    After the rearrangement procedure completes, what crate ends up on top of each stack?
    */
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
    /*
    --- Part Two ---

    As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

    Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 -
    it's a CrateMover 9001.

    The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick
    up and move multiple crates at once.

    Again considering the example above, the crates begin in the same configuration:

        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3
    Moving a single crate from stack 2 to stack 1 behaves the same as before:

    [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3
    However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this
    new configuration:

            [D]
            [N]
        [C] [Z]
        [M] [P]
     1   2   3
    Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

            [D]
            [N]
    [C]     [Z]
    [M]     [P]
     1   2   3
    Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

            [D]
            [N]
            [Z]
    [M] [C] [P]
     1   2   3
    In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

    Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final
    supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
    */
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
    fn solve() {
        println!("Day5 - First problem : {}", first().unwrap());
        println!("Day5 - Second problem : {}", second().unwrap())
    }
}
