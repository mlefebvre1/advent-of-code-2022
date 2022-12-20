use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use num::Integer;
use std::collections::VecDeque;

#[allow(dead_code)]
fn first() -> Result<String> {
    let data = std::fs::read_to_string("src/days/data/day11.txt")?;
    let monkeys: Result<Vec<Monkey>> = data.split("\n\n").map(Monkey::from_str).collect();
    let ans = run(20, monkeys?, true);
    Ok(ans.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let data = std::fs::read_to_string("src/days/data/day11.txt")?;
    let monkeys: Result<Vec<Monkey>> = data.split("\n\n").map(Monkey::from_str).collect();
    let ans = run(10000, monkeys?, false);
    Ok(ans.to_string())
}

#[derive(Debug)]
struct Monkey {
    starting_items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    nb_inspected_items: usize,
}
impl FromStr for Monkey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _id = lines.next().unwrap();
        let starting_items: VecDeque<usize> = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let operation = Operation::from_str(lines.next().unwrap())?;
        let other_lines = lines.join("\n");
        let test = Test::from_str(&other_lines)?;
        Ok(Self {
            starting_items,
            operation,
            test,
            nb_inspected_items: 0,
        })
    }
}

#[derive(Debug)]
enum Value {
    Old,
    Int(usize),
}
impl FromStr for Value {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s == "old" => Ok(Self::Old),
            _ => {
                let n = s.parse::<usize>()?;
                Ok(Self::Int(n))
            }
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Value),
    Mul(Value),
}
impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = s
            .trim()
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split_whitespace();
        let op = items.next().unwrap();
        let value = Value::from_str(items.next().unwrap())?;
        match op {
            op if op == "+" => Ok(Self::Add(value)),
            op if op == "*" => Ok(Self::Mul(value)),
            _ => Err(anyhow!("Unsupported operation {op}")),
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: usize,
    throw: Throw,
}
impl FromStr for Test {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let divisible_by = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse()?;
        let other_lines = lines.join("\n");
        let throw = Throw::from_str(&other_lines)?;
        Ok(Self {
            divisible_by,
            throw,
        })
    }
}

#[derive(Debug)]
struct Throw {
    if_true: usize,
    if_false: usize,
}
impl FromStr for Throw {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let if_true = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()?;
        let if_false = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()?;
        Ok(Self { if_true, if_false })
    }
}

fn run(nb_rounds: usize, mut monkeys: Vec<Monkey>, part1: bool) -> usize {
    let problem_domain = get_problem_domain(&monkeys);
    for _round in 0..nb_rounds {
        run_round(&mut monkeys, part1, problem_domain);
    }
    get_monkey_business(&monkeys)
}
fn run_round(monkeys: &mut [Monkey], part1: bool, modulus: usize) {
    // feels like C code but no choice if i use iterator, it will borrow mut the whole vector at once
    // and I wont be able to push to the new monkey..
    for monkey_id in 0..monkeys.len() {
        while let Some(item) = monkeys[monkey_id].starting_items.pop_front() {
            let worry_level = if part1 {
                apply_operation(item, &monkeys[monkey_id].operation) / 3
            } else {
                apply_operation(item, &monkeys[monkey_id].operation) % modulus
            };
            let test_result = test_worry_level(worry_level, &monkeys[monkey_id].test);
            let next_monkey_id = get_next_monkey_id(test_result, &monkeys[monkey_id]);
            monkeys[next_monkey_id]
                .starting_items
                .push_back(worry_level);
            monkeys[monkey_id].nb_inspected_items += 1;
        }
    }
}
fn apply_operation(item: usize, operation: &Operation) -> usize {
    match operation {
        Operation::Add(value) => item + get_value(item, value),
        Operation::Mul(value) => item * get_value(item, value),
    }
}
fn get_value(old: usize, value: &Value) -> usize {
    match value {
        Value::Old => old,
        Value::Int(n) => *n,
    }
}
fn test_worry_level(worry_level: usize, test: &Test) -> bool {
    worry_level.is_multiple_of(&test.divisible_by)
}
fn get_next_monkey_id(test_result: bool, monkey: &Monkey) -> usize {
    if test_result {
        monkey.test.throw.if_true
    } else {
        monkey.test.throw.if_false
    }
}
fn get_problem_domain(monkeys: &[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product()
}
fn get_monkey_business(monkeys: &[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|monkey| monkey.nb_inspected_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_part() {
        println!("Day11 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_second_part() {
        println!("Day11 - Second problem : \n{}", second().unwrap())
    }
}
