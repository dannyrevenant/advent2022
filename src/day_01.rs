pub fn answer() {
    let file_contents = include_str!("../input/day_01.txt");

    let mut values: Vec<u32> = file_contents
        .split("\n\n")
        .map(|elf| elf.lines().map(|meal| meal.parse::<u32>().unwrap()).sum())
        .collect();

    values.sort_by(|a, b| b.cmp(a));

    println!("Answer 01: {}", values[0]);

    println!("Answer 02: {}", values.iter().take(3).sum::<u32>());
}
