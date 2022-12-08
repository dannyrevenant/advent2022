use std::convert::TryFrom;

#[repr(u16)]
enum RoundScore {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl TryFrom<&str> for RoundScore {
    type Error = &'static str;

    fn try_from(intention: &str) -> Result<Self, Self::Error> {
        match intention {
            "X" => Ok(RoundScore::Lose),
            "Y" => Ok(RoundScore::Draw),
            "Z" => Ok(RoundScore::Win),
            _ => Err("Unknown intention"),
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

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(shape: &str) -> Result<Self, Self::Error> {
        match shape {
            "A" | "X" => Ok(Shape::Stone),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err("Unknown shape"),
        }
    }
}

impl Shape {
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

    let rounds = contents.split('\n');

    let total1 = rounds
        .clone()
        .fold(0, |acc, line| match line.split_once(' ') {
            Some((theirs, mine)) => {
                acc + Shape::try_from(mine).unwrap() as u16
                    + match (mine, theirs) {
                        _ if Shape::try_from(mine).unwrap().wins_against()
                            == Shape::try_from(theirs).unwrap() =>
                        {
                            RoundScore::Win as u16
                        }
                        _ if Shape::try_from(mine).unwrap().loses_to()
                            == Shape::try_from(theirs).unwrap() =>
                        {
                            RoundScore::Lose as u16
                        }
                        _ => RoundScore::Draw as u16,
                    }
            }
            None => acc,
        });

    println!("Answer 01: {}", total1);

    let total2 = rounds
        .clone()
        .fold(0, |acc, line| match line.split_once(' ') {
            Some((theirs, intention)) => {
                acc + RoundScore::try_from(intention).unwrap() as u16
                    + match RoundScore::try_from(intention).unwrap() {
                        RoundScore::Lose => Shape::try_from(theirs).unwrap().wins_against() as u16,
                        RoundScore::Draw => Shape::try_from(theirs).unwrap() as u16,
                        RoundScore::Win => Shape::try_from(theirs).unwrap().loses_to() as u16,
                    }
            }
            None => acc,
        });

    println!("Answer 02: {}", total2);
}
