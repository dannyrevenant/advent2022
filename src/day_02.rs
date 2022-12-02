#[repr(u16)]
enum RoundScore {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl RoundScore {
    fn from(intention: &str) -> Self {
        match intention {
            "X" => RoundScore::Lose,
            "Y" => RoundScore::Draw,
            "Z" => RoundScore::Win,
            _ => panic!("Unknown intention: {}", intention),
        }
    }
}

#[derive(PartialEq)]
#[repr(u16)]
enum Shape {
    Stone = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from(shape: &str) -> Self {
        match shape {
            "A" | "X" => Shape::Stone,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Unknown shape: {}", shape),
        }
    }

    fn wins_against(self) -> Shape {
        match self {
            Shape::Stone => Shape::Scissors,
            Shape::Paper => Shape::Stone,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn loses_to(self) -> Shape {
        match self {
            Shape::Stone => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Stone,
        }
    }
}

pub fn answer() {
    let contents = include_str!("../input/day_02.txt");

    let rounds = contents.split("\n");

    let total1 = rounds
        .clone()
        .fold(0, |acc, line| match line.split_once(" ") {
            Some((theirs, mine)) => acc
                + Shape::from(mine) as u16
                + match (mine, theirs) {
                    _ if Shape::from(mine).wins_against() == Shape::from(theirs) => RoundScore::Win,
                    _ if Shape::from(mine).loses_to() == Shape::from(theirs) => RoundScore::Lose,
                    _ => RoundScore::Draw,
                } as u16,
            None => acc,
        });

    println!("Answer 01: {}", total1);

    let total2 = rounds
        .clone()
        .fold(0, |acc, line| match line.split_once(" ") {
            Some((theirs, intention)) => acc
                + RoundScore::from(intention) as u16
                + match RoundScore::from(intention) {
                    RoundScore::Lose => Shape::from(theirs).wins_against(),
                    RoundScore::Draw => Shape::from(theirs),
                    RoundScore::Win => Shape::from(theirs).loses_to(),
                } as u16,
            None => acc,
        });

    println!("Answer 02: {}", total2);
}
