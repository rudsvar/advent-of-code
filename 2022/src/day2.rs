use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    fn beats(&self, shape: &Shape) -> Outcome {
        if self == shape {
            Outcome::Draw
        } else if self == &shape.next() {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }
    fn next(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
    fn prev(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

pub fn play_round(opponent: &Shape, you: &Shape) -> i32 {
    let outcome = you.beats(opponent);
    you.score() + outcome.score()
}

pub fn play_round_2(opponent: &Shape, outcome: &Outcome) -> i32 {
    let you = match outcome {
        Outcome::Draw => *opponent,
        Outcome::Loss => opponent.prev(),
        Outcome::Win => opponent.next(),
    };
    let outcome = you.beats(opponent);
    you.score() + outcome.score()
}

pub fn play_game(rounds: impl Iterator<Item = (Shape, Shape)>) -> i32 {
    rounds
        .map(|(opponent, you)| play_round(&opponent, &you))
        .sum()
}

pub fn play_game_2(rounds: impl Iterator<Item = (Shape, Outcome)>) -> i32 {
    rounds
        .map(|(opponent, you)| play_round_2(&opponent, &you))
        .sum()
}

pub fn parse_round(line: &str) -> (Shape, Shape) {
    let (a, b) = line.split_once(' ').expect("invalid format");
    let opponent = Shape::from_str(a).expect("invalid format");
    let you = Shape::from_str(b).expect("invalid format");
    (opponent, you)
}

pub fn parse_round_2(line: &str) -> (Shape, Outcome) {
    let (a, b) = line.split_once(' ').expect("invalid format");
    let opponent = Shape::from_str(a).expect("invalid format");
    let outcome = Outcome::from_str(b).expect("invalid format");
    (opponent, outcome)
}

pub fn parse_game(game: impl Iterator<Item = String>) -> impl Iterator<Item = (Shape, Shape)> {
    game.map(|line| parse_round(&line))
}

pub fn parse_game_2(game: impl Iterator<Item = String>) -> impl Iterator<Item = (Shape, Outcome)> {
    game.map(|line| parse_round_2(&line))
}

pub fn solve() -> i32 {
    let f = BufReader::new(File::open("data/day2.txt").unwrap());
    let lines = f.lines().map(|line| line.unwrap());
    let game = parse_game(lines);
    play_game(game)
}

pub fn solve_2() -> i32 {
    let f = BufReader::new(File::open("data/day2.txt").unwrap());
    let lines = f.lines().map(|line| line.unwrap());
    let game = parse_game_2(lines);
    play_game_2(game)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win() {
        assert_eq!(play_round(&Shape::Rock, &Shape::Paper), 8);
    }

    #[test]
    fn loss() {
        assert_eq!(play_round(&Shape::Paper, &Shape::Rock), 1);
    }

    #[test]
    fn draw() {
        assert_eq!(play_round(&Shape::Scissors, &Shape::Scissors), 6);
    }

    #[test]
    fn play_game_works() {
        let rounds = vec![
            (Shape::Rock, Shape::Paper),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissors, Shape::Scissors),
        ];
        assert_eq!(play_game(rounds.into_iter()), 15);
    }

    #[test]
    fn solve_works() {
        assert_eq!(solve(), 14827);
    }

    #[test]
    fn solve_2_works() {
        assert_eq!(solve_2(), 13889);
    }
}
