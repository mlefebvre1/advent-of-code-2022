use anyhow::{anyhow, Error, Ok, Result};
use std::{fs, str::FromStr};

#[allow(dead_code)]
fn first() -> Result<String> {
    /*
    --- Day 2: Rock Paper Scissors ---

    The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage,
    a giant Rock Paper Scissors tournament is already in progress.

    Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round,
    the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner
    for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players
    choose the same shape, the round instead ends in a draw.

    Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say
    will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C
    for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

    The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors.
    Winning every time would be suspicious, so the responses must have been carefully chosen.

    The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for
    each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for
    Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

    Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if
    you were to follow the strategy guide.

    For example, suppose you were given the following strategy guide:

    A Y
    B X
    C Z
    This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with
    a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with
    a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
    In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

    What would your total score be if everything goes exactly according to your strategy guide?
    */
    let data = fs::read_to_string("src/days/data/day2.txt")?;
    let total_score: usize = data
        .lines()
        .map(|line| Round::from_str(line).unwrap().score())
        .sum();
    Ok(total_score.to_string())
}

#[allow(dead_code)]
fn second() -> Result<String> {
    /*
    --- Part Two ---
    The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs
    to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

    The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends
    as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock.
    This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
    Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

    Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according
    to your strategy guide?
    */
    let data = fs::read_to_string("src/days/data/day2.txt")?;
    let total_score: usize = data
        .lines()
        .map(|line| RoundWithOutcomeGiven::from_str(line).unwrap().score())
        .sum();
    Ok(total_score.to_string())
}

#[derive(Debug, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl Choice {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
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
impl RoundOutcome {
    fn score(&self) -> usize {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Won => 6,
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
impl Round {
    fn score(&self) -> usize {
        let round_outcome = RoundOutcome::from(self.clone()).score();
        let choice_value = self.you.clone().score();
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
impl RoundWithOutcomeGiven {
    fn score(self) -> usize {
        let choice_value = Choice::from(self.clone()).score();
        choice_value + self.round_outcome.score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        println!("Day2 - First problem : {}", first().unwrap());
        println!("Day2 - Second problem : {}", second().unwrap())
    }
}
