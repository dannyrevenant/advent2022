fn get_map(contents: &str) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for line in contents.lines() {
        let mut row: Vec<u32> = vec![];
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap());
        }
        result.push(row);
    }
    result
}

fn get_results(contents: &str) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let mut visible = 0;

    let map = get_map(contents);

    for (index_line, line) in map.iter().enumerate() {
        let len = line.len();

        if index_line == 0 || index_line == len - 1 {
            visible += len;
            continue;
        }

        'outer: for (index_row, char) in line.iter().enumerate() {
            if index_row == 0 || index_row == len - 1 {
                visible += 1;
                continue;
            }

            for (i, c) in line.iter().enumerate() {
                if i == index_row {
                    visible += 1;
                    continue 'outer;
                }
                if c >= char {
                    break;
                }
            }

            for (i, c) in line.iter().enumerate() {
                if i <= index_row {
                    continue;
                }

                if c >= char {
                    break;
                }

                if i == len - 1 {
                    visible += 1;
                    continue 'outer;
                }
            }

            for (i, line_to_parse) in map.iter().enumerate() {
                if i == index_line {
                    visible += 1;
                    continue 'outer;
                }

                if &line_to_parse[index_row] >= char {
                    break;
                }
            }

            for (i, line_to_parse) in map.iter().enumerate() {
                if i <= index_line {
                    continue;
                }

                if &line_to_parse[index_row] >= char {
                    break;
                }

                if i == map.len() - 1 {
                    visible += 1;
                    continue 'outer;
                }
            }
        }
    }

    let mut max_score = 0;

    for (index_line, line) in map.iter().enumerate() {
        if index_line == 0 || index_line == map.len() - 1 {
            continue;
        }

        let len = line.len();
        for (index_row, char) in line.iter().enumerate() {
            if index_row == 0 || index_row == len - 1 {
                continue;
            }

            //left
            let mut left = 0;
            for (i, c) in line.iter().enumerate() {
                if i == index_row {
                    break;
                }

                left += 1;

                if c >= char {
                    left = 0;
                }
            }

            //right
            let mut right = 0;
            for (i, c) in line.iter().enumerate() {
                if i <= index_row {
                    continue;
                }

                right += 1;

                if c >= char {
                    break;
                }
            }

            //up
            let mut up = 0;
            for (i, line_to_parse) in map.iter().enumerate() {
                if i == index_line {
                    break;
                }

                up += 1;

                if &line_to_parse[index_row] >= char {
                    up = 0;
                }
            }

            //down
            let mut down = 0;
            for (i, line_to_parse) in map.iter().enumerate() {
                if i <= index_line {
                    continue;
                }

                down += 1;

                if &line_to_parse[index_row] >= char {
                    break;
                }
            }

            let score = left * right * up * down;

            if score > max_score {
                max_score = score;
            }
        }
    }

    Ok((visible.try_into().unwrap(), max_score))
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

    let expected_output = (1823, 211680);

    let actual_output = get_results(input).unwrap();

    assert_eq!(actual_output, expected_output);
}
