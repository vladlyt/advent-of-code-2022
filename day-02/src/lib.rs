use std::str::FromStr;

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Round {
    opponent: Move,
    me: Move,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(format!(
                "Length of the input must be equal to 3, got {}",
                s.len()
            ));
        }
        let parts: Vec<&str> = s.split(" ").collect();
        let opponent = match parts[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            v => {
                return Err(format!("Invalid argument, got {}", v));
            }
        };
        let me = match parts[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            v => {
                return Err(format!("Invalid argument, got {}", v));
            }
        };
        Ok(Self {
            opponent: opponent,
            me: me,
        })
    }
}

impl Round {
    fn score(&self) -> u32 {
        self.me as u32
            + match (self.opponent, self.me) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 6,
                (Move::Rock, Move::Scissors) => 0,
                (Move::Paper, Move::Rock) => 0,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissors) => 6,
                (Move::Scissors, Move::Rock) => 6,
                (Move::Scissors, Move::Paper) => 0,
                (Move::Scissors, Move::Scissors) => 3,
            }
    }
}

struct PredictRound {
    opponent: Move,
    me: Move,
}

impl FromStr for PredictRound {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(format!(
                "Length of the input must be equal to 3, got {}",
                s.len()
            ));
        }
        let parts: Vec<&str> = s.split(" ").collect();
        let opponent = match parts[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            v => {
                return Err(format!("Invalid argument, got {}", v));
            }
        };

        let me = match parts[1] {
            "X" => match opponent {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            "Y" => opponent,
            "Z" => match opponent {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            v => {
                return Err(format!("Invalid argument, got {}", v));
            }
        };
        Ok(Self {
            opponent: opponent,
            me: me,
        })
    }
}

impl PredictRound {
    fn score(&self) -> u32 {
        self.me as u32
            + match (self.opponent, self.me) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 6,
                (Move::Rock, Move::Scissors) => 0,
                (Move::Paper, Move::Rock) => 0,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissors) => 6,
                (Move::Scissors, Move::Rock) => 6,
                (Move::Scissors, Move::Paper) => 0,
                (Move::Scissors, Move::Scissors) => 3,
            }
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Round>().unwrap().score())
        .sum::<u32>()
        .to_string()
}
pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<PredictRound>().unwrap().score())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
