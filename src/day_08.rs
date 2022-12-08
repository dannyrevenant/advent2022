fn get_map(contents: &str) -> Vec<Vec<u8>> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn get_results(contents: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let map = get_map(contents);

    let visible = map
        .iter()
        .enumerate()
        .map(|(line_pos, line)| {
            let len = line.len();
            if line_pos == 0 || line_pos == len - 1 {
                return len;
            }

            line.iter()
                .enumerate()
                .filter(|(char_pos, char)| {
                    if *char_pos == 0 || *char_pos == len - 1 {
                        return true;
                    }

                    if line[0..*char_pos].iter().all(|item| item < char) {
                        return true;
                    }

                    if line[char_pos + 1..].iter().all(|item| item < char) {
                        return true;
                    }

                    if map[0..line_pos]
                        .iter()
                        .all(|line_to_parse| &line_to_parse[*char_pos] < char)
                    {
                        return true;
                    }

                    if map[line_pos + 1..]
                        .iter()
                        .all(|line_to_parse| &line_to_parse[*char_pos] < char)
                    {
                        return true;
                    }

                    false
                })
                .count()
        })
        .sum();

    let max_score = map
        .iter()
        .enumerate()
        .map(|(line_pos, line)| {
            let len = line.len();
            if line_pos == 0 || line_pos == len - 1 {
                return 0;
            }

            return line
                .iter()
                .enumerate()
                .map(|(char_pos, char)| {
                    if char_pos == 0 || char_pos == len - 1 {
                        return 0;
                    }

                    //left
                    let left = line[0..char_pos]
                        .iter()
                        .rev()
                        .take_while(|c| c < &char)
                        .count();

                    //right
                    let right = line[char_pos + 1..]
                        .iter()
                        .take_while(|c| c < &char)
                        .count();

                    //up
                    let up = map[0..line_pos]
                        .iter()
                        .rev()
                        .take_while(|line_to_parse| line_to_parse[char_pos] < *char)
                        .count();

                    //down
                    let down = map[line_pos + 1..]
                        .iter()
                        .take_while(|line_to_parse| line_to_parse[char_pos] < *char)
                        .count();

                    left * right * up * (down + 1)
                })
                .max()
                .unwrap();
        })
        .max()
        .unwrap();

    Ok((visible, max_score))
}

pub fn answer() -> Result<(), Box<dyn std::error::Error>> {
    let contents = include_str!("../input/day_08.txt");

    let (visible, max_score) = get_results(contents)?;

    println!("Answer 01: {}", visible);

    println!("Answer 02: {}", max_score);

    Ok(())
}

#[test]
fn test_answer() {
    let input = include_str!("../input/day_08.txt");
    //     let input = "30373
    // 25512
    // 65332
    // 33549
    // 35390";

    let expected_output = (1823, 211680);

    let actual_output = get_results(input).unwrap();

    assert_eq!(actual_output, expected_output);
}
