use phf::phf_map;
use std::{cmp::Ordering, collections::HashMap};

static ORDER_ONE: phf::Map<char, usize> = phf_map! {
    'A' => 0,
    'K' => 1,
    'Q' => 2,
    'J' => 3,
    'T' => 4,
    '9' => 5,
    '8' => 6,
    '7' => 7,
    '6' => 8,
    '5' => 9,
    '4' => 10,
    '3' => 11,
    '2' => 12,
};

static ORDER_TWO: phf::Map<char, usize> = phf_map! {
    'A' => 0,
    'K' => 1,
    'Q' => 2,
    'T' => 3,
    '9' => 4,
    '8' => 5,
    '7' => 6,
    '6' => 7,
    '5' => 8,
    '4' => 9,
    '3' => 10,
    '2' => 11,
    'J' => 12,
};

#[derive(Debug)]
enum HandKind {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    Two = 3,
    One = 2,
    High = 1,
}

fn count_chars(str: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for c in str.chars() {
        let entry = result.entry(c).or_insert(0);
        *entry += 1;
    }

    result
}

fn determine_kind(char_count: &HashMap<char, usize>, is_part_two: bool) -> HandKind {
    let kind = match char_count.len() {
        1 => HandKind::Five,
        2 => {
            if char_count.values().any(|v| v == &3) {
                HandKind::Full
            } else {
                HandKind::Four
            }
        }
        3 => {
            if char_count.values().any(|v| v == &3) {
                HandKind::Three
            } else {
                HandKind::Two
            }
        }
        4 => HandKind::One,
        5 => HandKind::High,
        _ => unreachable!(),
    };

    let j_count = char_count.get(&'J').unwrap_or(&0);

    if is_part_two && j_count > &0 && j_count < &5 {
        let m = char_count
            .iter()
            .reduce(|agg, e| {
                if agg.0 == &'J' {
                    return e;
                }

                if e.0 == &'J' {
                    return agg;
                }

                if e.1 > agg.1 {
                    return e;
                }

                agg
            })
            .unwrap();

        let mut new_char_count = char_count.clone();
        new_char_count.entry(*m.0).and_modify(|x| *x += j_count);
        new_char_count.remove(&'J');

        return determine_kind(&new_char_count, false);
    }

    kind
}

fn compare_strings(s1: &str, s2: &str, order: &phf::Map<char, usize>) -> Ordering {
    for (char_a, char_b) in s1.chars().zip(s2.chars()) {
        let pos_a = order.get(&char_a).unwrap();
        let pos_b = order.get(&char_b).unwrap();
        match pos_b.cmp(pos_a) {
            Ordering::Equal => continue,
            non_equal => return non_equal,
        }
    }

    unreachable!()
}

fn solve_part_one(lines: &[&str]) {
    let mut pairs = lines
        .iter()
        .map(|x| {
            let (hand, raw_bid) = x.split_once(' ').unwrap();

            (hand, raw_bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    pairs.sort_by(|a, b| {
        let a_hand = a.0;
        let b_hand = b.0;
        let a_count = count_chars(a_hand);
        let b_count = count_chars(b_hand);
        let a_kind = determine_kind(&a_count, false);
        let b_kind = determine_kind(&b_count, false);

        let kind_cmp = (a_kind as usize).cmp(&(b_kind as usize));
        if kind_cmp == Ordering::Equal {
            return compare_strings(a_hand, b_hand, &ORDER_ONE);
        }

        kind_cmp
    });

    let mut sum = 0;

    for (idx, (_, bid)) in pairs.iter().enumerate() {
        sum += (idx + 1) * bid;
    }

    println!("Part 1: {sum}");
}

fn solve_part_two(lines: &[&str]) {
    let mut pairs = lines
        .iter()
        .map(|x| {
            let (hand, raw_bid) = x.split_once(' ').unwrap();

            (hand, raw_bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    pairs.sort_by(|a, b| {
        let a_hand = a.0;
        let b_hand = b.0;
        let a_count = count_chars(a_hand);
        let b_count = count_chars(b_hand);
        let a_kind = determine_kind(&a_count, true);
        let b_kind = determine_kind(&b_count, true);

        let kind_cmp = (a_kind as usize).cmp(&(b_kind as usize));
        if kind_cmp == Ordering::Equal {
            return compare_strings(a_hand, b_hand, &ORDER_TWO);
        }

        kind_cmp
    });

    let mut sum = 0;

    for (idx, (_, bid)) in pairs.iter().enumerate() {
        sum += (idx + 1) * bid;
    }

    println!("Part 2: {sum}");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    solve_part_one(&lines);
    solve_part_two(&lines);
}
