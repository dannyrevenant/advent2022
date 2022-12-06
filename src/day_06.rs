use std::error::Error;
use std::collections::HashSet;

fn find_start(content: &str, size: usize) -> Result<usize, Box<dyn Error>> {
    let result = content.chars()
        .collect::<Vec<char>>()
        .windows(size)
        .map(|window| -> HashSet<char> { HashSet::from_iter(window.to_vec()) })
        .position(|window| window.len() == size);

    match result {
        Some(result) => Ok(result + size),
        _ => Err("Not found".into())
    }
}

pub fn answer() -> Result<(), Box<dyn Error>> {
    let content = include_str!("../input/day_06.txt");

    let answer_1 = find_start(content, 4)?;
    println!("Answer 01: {}", answer_1);

    let answer_2 = find_start(content, 14)?;
    println!("Answer 02: {}", answer_2);

    Ok(())
}
