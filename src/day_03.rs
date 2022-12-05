use std::collections::HashMap;

fn get_priority(item: &char) -> usize {
    let map = {
        let mut lower: HashMap<char, usize> = ('a'..='z')
            .into_iter()
            .map(|c| (c, c as usize - 96))
            .collect();

        let upper: HashMap<char, usize> = ('A'..='Z')
            .into_iter()
            .map(|c| (c, c as usize - 38))
            .collect();
        lower.extend(upper);
        lower
    };

    *map.get(item).unwrap()
}

pub fn answer() {
    let lines: Vec<&str> = include_str!("../input/day_03.txt").lines().collect();

    println!(
        "Answer 01: {:?}",
        lines
            .iter()
            .map(|r| {
                let (comp1, comp2) = r.split_at(r.len() / 2);
                for c in comp1.chars() {
                    if comp2.contains(c) {
                        return get_priority(&c);
                    }
                }
                panic!("Couldn't find a pair.");
            })
            .sum::<usize>()
    );

    println!(
        "Answer 02: {:?}",
        lines
            .chunks(3)
            .map(|chunk| {
                for c in chunk[0].chars() {
                    if chunk[1].contains(c) && chunk[2].contains(c) {
                        return get_priority(&c);
                    }
                }
                panic!("Couldn't find common item.");
            })
            .sum::<usize>()
    )
}
