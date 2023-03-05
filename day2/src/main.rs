use anyhow::{anyhow, Error, Ok, Result};
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

fn first() -> Result<String> {
    let data = fs::read_to_string("day2/data/day2.txt")?;
    let total_score: usize = data
        .lines()
        .map(|line| {
            let score: usize = Round::from_str(line).unwrap().into();
            score
        })
        .sum();
    Ok(total_score.to_string())
}

fn second() -> Result<String> {
    let data = fs::read_to_string("day2/data/day2.txt")?;
    let total_score: usize = data
        .lines()
        .map(|line| {
            let score: usize = RoundWithOutcomeGiven::from_str(line).unwrap().into();
            score
        })
        .sum();
    Ok(total_score.to_string())
}

#[derive(Debug, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl From<Choice> for usize {
    fn from(choice: Choice) -> Self {
        match choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}
impl FromStr for Choice {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(anyhow!("Bad Rock Paper Scissor choice")),
        }
    }
}
impl From<RoundWithOutcomeGiven> for Choice {
    fn from(round: RoundWithOutcomeGiven) -> Self {
        match (round.opponent_choice, round.round_outcome) {
            (Self::Rock, RoundOutcome::Lost) => Self::Scissors,
            (Self::Rock, RoundOutcome::Draw) => Self::Rock,
            (Self::Rock, RoundOutcome::Won) => Self::Paper,
            (Self::Paper, RoundOutcome::Lost) => Self::Rock,
            (Self::Paper, RoundOutcome::Draw) => Self::Paper,
            (Self::Paper, RoundOutcome::Won) => Self::Scissors,
            (Self::Scissors, RoundOutcome::Lost) => Self::Paper,
            (Self::Scissors, RoundOutcome::Draw) => Self::Scissors,
            (Self::Scissors, RoundOutcome::Won) => Self::Rock,
        }
    }
}

#[derive(Clone, Debug)]
enum RoundOutcome {
    Lost,
    Draw,
    Won,
}
impl From<RoundOutcome> for usize {
    fn from(outcome: RoundOutcome) -> Self {
        match outcome {
            RoundOutcome::Lost => 0,
            RoundOutcome::Draw => 3,
            RoundOutcome::Won => 6,
        }
    }
}
impl From<Round> for RoundOutcome {
    fn from(round: Round) -> Self {
        match (round.you, round.opponent) {
            (Choice::Rock, Choice::Rock) => Self::Draw,
            (Choice::Rock, Choice::Paper) => Self::Lost,
            (Choice::Rock, Choice::Scissors) => Self::Won,
            (Choice::Paper, Choice::Rock) => Self::Won,
            (Choice::Paper, Choice::Paper) => Self::Draw,
            (Choice::Paper, Choice::Scissors) => Self::Lost,
            (Choice::Scissors, Choice::Rock) => Self::Lost,
            (Choice::Scissors, Choice::Paper) => Self::Won,
            (Choice::Scissors, Choice::Scissors) => Self::Draw,
        }
    }
}
impl FromStr for RoundOutcome {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lost),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Won),
            _ => Err(anyhow!("Wrong round outcome")),
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    you: Choice,
    opponent: Choice,
}
impl FromStr for Round {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let opponent = Choice::from_str(s.next().unwrap())?;
        let you = Choice::from_str(s.next().unwrap())?;
        Ok(Self { opponent, you })
    }
}
impl From<Round> for usize {
    fn from(round: Round) -> Self {
        let round_outcome: usize = RoundOutcome::from(round.clone()).into();
        let choice_value: usize = round.you.into();
        round_outcome + choice_value
    }
}

#[derive(Debug, Clone)]
struct RoundWithOutcomeGiven {
    round_outcome: RoundOutcome,
    opponent_choice: Choice,
}
impl FromStr for RoundWithOutcomeGiven {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let opponent_choice = Choice::from_str(s.next().unwrap())?;
        let round_outcome = RoundOutcome::from_str(s.next().unwrap())?;
        Ok(Self {
            round_outcome,
            opponent_choice,
        })
    }
}
impl From<RoundWithOutcomeGiven> for usize {
    fn from(round: RoundWithOutcomeGiven) -> Self {
        let choice_value: usize = Choice::from(round.clone()).into();
        let round_outcome: usize = round.round_outcome.into();
        choice_value + round_outcome
    }
}
