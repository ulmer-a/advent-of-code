pub fn main(input: String) -> (u64, u64) {
    let total_score = input
        .lines()
        .map(|line| parse_line::<Turn, Turn>(line))
        .map(|(t1, t2)| score(t1, t2))
        .sum();

    let score_with_directive: u64 = input
        .lines()
        .map(|line| parse_line::<Turn, Directive>(line))
        .map(|(t, d)| score(d.find_turn(t), t))
        .sum();

    (total_score, score_with_directive)
}

#[derive(Copy, Clone, PartialEq)]
enum Turn {
    Rock,
    Paper,
    Scissors,
}

impl Turn {
    fn base_score(&self) -> u64 {
        match self {
            Turn::Rock => 1,
            Turn::Paper => 2,
            Turn::Scissors => 3,
        }
    }

    fn is_win(self, other: Turn) -> Option<bool> {
        match (self, other) {
            (a, b) if a == b => None,
            (Turn::Rock, Turn::Scissors) => Some(true),
            (Turn::Scissors, Turn::Paper) => Some(true),
            (Turn::Paper, Turn::Rock) => Some(true),
            _ => Some(false),
        }
    }

    fn into_losing(self) -> Turn {
        match self {
            Turn::Rock => Turn::Scissors,
            Turn::Paper => Turn::Rock,
            Turn::Scissors => Turn::Paper,
        }
    }

    fn into_draw(self) -> Turn {
        self
    }

    fn into_win(self) -> Turn {
        match self {
            Turn::Rock => Turn::Paper,
            Turn::Paper => Turn::Scissors,
            Turn::Scissors => Turn::Rock,
        }
    }
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Turn::Rock,
            "B" | "Y" => Turn::Paper,
            "C" | "Z" => Turn::Scissors,
            _ => panic!("unexpected character"),
        }
    }
}

enum Directive {
    Lose,
    Draw,
    Win,
}

impl Directive {
    fn find_turn(self, opponents_turn: Turn) -> Turn {
        match (self, opponents_turn) {
            (Directive::Lose, t) => t.into_losing(),
            (Directive::Draw, t) => t.into_draw(),
            (Directive::Win, t) => t.into_win(),
        }
    }
}

impl From<&str> for Directive {
    fn from(value: &str) -> Self {
        match value {
            "X" => Directive::Lose,
            "Y" => Directive::Draw,
            "Z" => Directive::Win,
            _ => panic!("unexpected character"),
        }
    }
}

fn parse_line<'a, T1: From<&'a str>, T2: From<&'a str>>(line: &'a str) -> (T1, T2) {
    let (opp, my) = line.split_once(" ").unwrap();
    (T1::from(opp), T2::from(my))
}

fn score(my_turn: Turn, opponents_turn: Turn) -> u64 {
    my_turn.base_score()
        + match my_turn.is_win(opponents_turn) {
            None => 3,
            Some(true) => 6,
            Some(false) => 0,
        }
}
