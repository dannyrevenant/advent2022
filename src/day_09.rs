use std::{collections::HashSet, convert::TryFrom};

#[derive(Debug, Copy, Clone)]
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

fn update_head(mut head: &mut Point, direction: Direction) {
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

fn update_tail(head: Point, mut tail: &mut Point) {
    let (delta_x, delta_y) = (head.x - tail.x, head.y - tail.y);
    if delta_x.abs() == 2 || delta_y.abs() == 2 {
        tail.x += delta_x.signum();
        tail.y += delta_y.signum();
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
        .flat_map(|(left, right)| {
            vec![Direction::try_from(left).unwrap(); right.parse::<usize>().unwrap()]
        })
        .for_each(|direction| {
            update_head(&mut rope.knots[0], direction);
            for i in 1..rope.knots.len() {
                update_tail(rope.knots[i - 1].clone(), &mut rope.knots[i]);
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
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    let expected_output = 36;

    let actual_output = get_results(input, 10).unwrap();

    assert_eq!(actual_output, expected_output);
}
