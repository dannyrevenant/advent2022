use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::error::Error;

struct Map(BTreeMap<usize, Vec<char>>);

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(setup: &str) -> Result<Self, Self::Error> {
        let mut lines = setup.lines().into_iter().rev();

        let mut map: BTreeMap<usize, Vec<char>> = lines
            .next()
            .ok_or("Could not get first line")?
            .split_whitespace()
            .map(|column| (column.parse().unwrap(), vec![]))
            .collect();

        lines.for_each(|row| {
            row.chars().skip(1).step_by(4).enumerate()
                .filter(|(_, char)| ![' ', '[', ']'].contains(char))
                .for_each(|(index, char)| {
                    map.get_mut(&(index + 1)).unwrap().push(char);
                });
        });

        Ok(Map(map))
    }
}

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(row: &str) -> Result<Self, Self::Error> {
        let data: Vec<usize> = row
            .split_whitespace()
            .skip(1)
            .step_by(1)
            .flat_map(|e| e.parse())
            .collect();

        Ok(Move {
            quantity: data[0],
            from: data[1],
            to: data[2],
        })
    }
}

fn get_answer(mut map: Map) -> String {
    map.0
        .iter_mut()
        .flat_map(|(_, column)| column.pop())
        .collect()
}

pub fn answer() -> Result<(), Box<dyn Error>> {
    let content = include_str!("../input/day_05.txt");

    let (setup, plan) = content.split_once("\n\n").ok_or("Failed to split")?;

    let mut map = Map::try_from(setup)?;

    for row in plan.trim().lines() {
        let the_move = Move::try_from(row)?;

        let len = map.0.get(&the_move.from).ok_or("Could not get map structure")?.len();

        map.0
            .get_mut(&the_move.from)
            .ok_or("Could not get map structure")?
            .split_off(len - the_move.quantity)
            .iter()
            .rev()
            .try_for_each(|item| match map.0.get_mut(&the_move.to) {
                Some(map) => Ok(map.push(*item)),
                None => Err("Could not get map structure"),
            })?;
    }

    println!("Answer 01: {}", get_answer(map));

    let mut map2 = Map::try_from(setup)?;

    for row in plan.trim().lines() {
        let the_move = Move::try_from(row)?;

        let len = map2.0.get(&the_move.from).ok_or("Could not get map structure")?.len();

        map2.0
            .get_mut(&the_move.from)
            .ok_or("Could not get map structure")?
            .split_off(len - the_move.quantity)
            .iter()
            .try_for_each(|item| match map2.0.get_mut(&the_move.to) {
                Some(map) => Ok(map.push(*item)),
                None => Err("Gutted"),
            })?;
    }

    println!("Answer 02: {}", get_answer(map2));

    Ok(())
}
