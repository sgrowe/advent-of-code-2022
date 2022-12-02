use std::str::FromStr;

use crate::utils::parse_as;

pub fn main(input: &str) {
    let Scores { part_one, part_two } = total_scores(input);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

#[derive(Debug, Clone, Copy)]
struct Scores {
    part_one: usize,
    part_two: usize,
}

fn total_scores(input: &str) -> Scores {
    let mut part_one = 0;
    let mut part_two = 0;

    for l in input.lines() {
        part_one += parse_as::<Round>(l).score();
        part_two += parse_as::<RoundV2>(l).as_round().score();
    }

    Scores { part_one, part_two }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: RPS,
    you: RPS,
}

impl Round {
    fn score(self) -> usize {
        self.you.score() + self.you.result_against(self.opponent).score()
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let opponent = RPS::from_char(chars.next().unwrap());

        // Space char
        chars.next().unwrap();

        let you = RPS::from_char(chars.next().unwrap());

        Ok(Round { opponent, you })
    }
}

#[derive(Debug, Clone, Copy)]
struct RoundV2 {
    opponent: RPS,
    result: RPSResult,
}

impl RoundV2 {
    fn as_round(self) -> Round {
        let you = self.result.corresponding_choice_against(self.opponent);

        Round {
            opponent: self.opponent,
            you,
        }
    }
}

impl FromStr for RoundV2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let opponent = RPS::from_char(chars.next().unwrap());

        // Space char
        chars.next().unwrap();

        let result = RPSResult::from_char(chars.next().unwrap());

        Ok(RoundV2 { opponent, result })
    }
}

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn result_against(self, other: RPS) -> RPSResult {
        match (self, other) {
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
                RPSResult::Win
            }
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
                RPSResult::Loss
            }
            _ => RPSResult::Draw,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Scissors,
            _ => panic!("Unknown char: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RPSResult {
    Win,
    Draw,
    Loss,
}

impl RPSResult {
    fn score(self) -> usize {
        match self {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Loss => 0,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'X' => RPSResult::Loss,
            'Y' => RPSResult::Draw,
            'Z' => RPSResult::Win,
            _ => panic!("Unknown char: {}", c),
        }
    }

    fn corresponding_choice_against(self, opponent: RPS) -> RPS {
        match self {
            RPSResult::Draw => opponent,
            RPSResult::Win => match opponent {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            RPSResult::Loss => match opponent {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    use test_case::test_case;

    const SAMPLE: &str = "
A Y
B X
C Z
";

    #[test_case("A Y", 8)]
    #[test_case("B X", 1)]
    #[test_case("C Z", 6)]
    fn test_round_scores(round_input: &str, score: usize) {
        let round = parse_as::<Round>(round_input);

        assert_eq!(round.score(), score);
    }

    #[test]
    fn part_one() {
        assert_eq!(total_scores(SAMPLE.trim()).part_one, 15);

        assert_eq!(total_scores(&read_input(2)).part_one, 12156);
    }

    #[test_case("A Y", 4)]
    #[test_case("B X", 1)]
    #[test_case("C Z", 7)]
    fn test_round_scores_part_two(round_input: &str, score: usize) {
        let roundv2 = parse_as::<RoundV2>(round_input);

        assert_eq!(roundv2.as_round().score(), score);
    }

    #[test]
    fn part_two() {
        assert_eq!(total_scores(SAMPLE.trim()).part_two, 12);

        assert_eq!(total_scores(&read_input(2)).part_two, 10835);
    }
}
