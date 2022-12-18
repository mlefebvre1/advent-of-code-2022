use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Error, Result};

#[allow(dead_code)]
fn first() -> Result<String> {
    let instructions = get_instructions_from_file()?;
    let mut cpu = Cpu::new();
    cpu.run(instructions);
    Ok(cpu.sum_signal_strenght.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    let instructions = get_instructions_from_file()?;
    let mut cpu = Cpu::new();
    cpu.run(instructions);
    Ok(cpu.crt.to_string())
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}
impl Instruction {
    fn nb_clk_cycle(&self) -> u64 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}
impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.starts_with("addx") => {
                let mut s = s.split_whitespace();
                let _ = s.next();
                let value = s.next().unwrap().parse::<i64>()?;
                Ok(Self::Addx(value))
            }
            s if s.starts_with("noop") => Ok(Self::Noop),
            _ => Err(anyhow!("Unrecognized CPU instruction")),
        }
    }
}

fn get_instructions_from_file() -> Result<Vec<Instruction>> {
    let data = std::fs::read_to_string("src/days/data/day10.txt")?;
    data.lines().map(Instruction::from_str).collect()
}

struct Cpu {
    clk_cycle: u64,
    reg_x_value: i64,
    sum_signal_strenght: i64,
    crt: Crt,
}
impl Cpu {
    fn new() -> Self {
        Self {
            clk_cycle: 1,
            reg_x_value: 1,
            sum_signal_strenght: 0,
            crt: Crt::default(),
        }
    }
    fn run(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }
    fn execute_instruction(&mut self, instruction: Instruction) {
        let nb_clk_cycles = instruction.nb_clk_cycle();
        for cycle in 0..nb_clk_cycles {
            self.update_signal_strengh_sum();
            self.crt.draw_pixel(self.clk_cycle, self.reg_x_value);
            match instruction {
                Instruction::Addx(v) => {
                    if cycle == nb_clk_cycles - 1 {
                        self.reg_x_value += v;
                    }
                }
                Instruction::Noop => (),
            }
            self.clk_cycle += 1;
        }
    }
    fn update_signal_strengh_sum(&mut self) {
        if (self.clk_cycle + 20) % 40 == 0 {
            self.sum_signal_strenght += (self.clk_cycle as i64) * self.reg_x_value;
        }
    }
}

#[derive(Debug)]
struct Crt([[bool; 40]; 6]);
impl Crt {
    fn draw_pixel(&mut self, cpu_clk_cycle: u64, reg_value: i64) {
        let x = (cpu_clk_cycle - 1) % 40;
        let y = (cpu_clk_cycle - 1) / 40;
        let sprite = (reg_value - 1)..=(reg_value + 1);
        if sprite.contains(&(x as i64)) {
            self.0[y as usize][x as usize] = true;
        }
    }
}
impl Default for Crt {
    fn default() -> Self {
        Self([[false; 40]; 6])
    }
}
impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();
        for row in self.0 {
            for col in row {
                let c = if col { '#' } else { '.' };
                s.push(c);
            }
            s.push('\n');
        }
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_part() {
        println!("Day10 - First problem : {}", first().unwrap());
    }
    #[test]
    fn solve_second_part() {
        println!("Day10 - Second problem : \n{}", second().unwrap())
    }
}
