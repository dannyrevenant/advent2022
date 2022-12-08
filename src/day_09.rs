use std::{collections::HashSet, convert::TryFrom};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = &'static str;

    fn try_from(intention: &str) -> Result<Self, Self::Error> {
        match intention {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err("Unknown intention"),
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Point>,
}

#[derive(Default, Debug, Clone)]
struct Point {
    x: i16,
    y: i16,
    previous_positions: HashSet<(i16, i16)>,
}

fn update_head<'a>(mut head: &'a mut Point, direction: &Direction) {
    match direction {
        Direction::Left => {
            head.x -= 1;
        }
        Direction::Right => {
            head.x += 1;
        }
        Direction::Up => {
            head.y += 1;
        }
        Direction::Down => {
            head.y -= 1;
        }
    }
}

fn update_tail<'a>(head: &'a Point, mut tail: &'a mut Point, direction: &Direction) {
    let x_range = head.x - 1..=head.x + 1;
    let y_range = head.y - 1..=head.y + 1;

    if !x_range
        .flat_map(|x| y_range.clone().map(move |y| (x, y)))
        .any(|tuple| tuple == (tail.x, tail.y))
    {
        tail.x = head.x;
        tail.y = head.y;
        match direction {
            Direction::Left => {
                tail.x += 1;
            }
            Direction::Right => {
                tail.x -= 1;
            }
            Direction::Up => {
                tail.y -= 1;
            }
            Direction::Down => {
                tail.y += 1;
            }
        }
    }

    tail.previous_positions.insert((tail.x, tail.y));
}

fn get_results(contents: &str, length: u8) -> Result<usize, Box<dyn std::error::Error>> {
    let mut rope = Rope { knots: vec![] };

    for _ in 0..length {
        rope.knots.push(Point::default());
    }

    contents
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(left, right)| {
            (
                Direction::try_from(left).unwrap(),
                right.parse::<i16>().unwrap(),
            )
        })
        .for_each(|(direction, amount)| {
            for _ in 1..=amount {
                update_head(&mut rope.knots[0], &direction);
                for i in 1..rope.knots.len() {
                    update_tail(&rope.knots[i - 1].clone(), &mut rope.knots[i], &direction);
                }
            }
        });

    Ok(rope.knots.last().unwrap().previous_positions.len())
}

pub fn answer() -> Result<(), Box<dyn std::error::Error>> {
    let contents = include_str!("../input/day_09.txt");
    let one = get_results(contents, 2)?;
    println!("Answer 01: {one}");
    let two = get_results(contents, 10)?;
    println!("Answer 02: {two}");
    Ok(())
}

#[test]
fn test_answer() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let expected_output = 13;

    let actual_output = get_results(input, 2).unwrap();

    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_answer_2() {
    // let input = include_str!("../input/day_08.txt");
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let expected_output = 36;

    let actual_output = get_results(input, 10).unwrap();

    assert_eq!(actual_output, expected_output);
}
