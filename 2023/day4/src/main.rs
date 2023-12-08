use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    wins: usize,
}

fn parse_card(line: &String) -> Card {
    let (id_part, other) = line.split_once(": ").unwrap();
    let raw_id = id_part.split_once(' ').unwrap().1;
    let id = raw_id.trim().parse::<usize>().unwrap();
    let (winning, current) = other.trim().split_once(" | ").unwrap();
    let winning = winning
        .split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    let current = current
        .split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();

    let wins = current.intersection(&winning).count();

    Card { id, wins }
}

fn solve_part_one(lines: &Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let matching_count = parse_card(line);

        let mut local_sum = 0;
        for n in 0..matching_count.wins {
            if n == 0 {
                local_sum = 1;
            } else {
                local_sum *= 2;
            }
        }

        sum += local_sum;
    }

    println!("Part 1: {sum}");
}

fn solve_part_two(lines: &Vec<String>) {
    let orig_cards: Vec<Card> = lines.iter().map(parse_card).collect();
    let mut cards: Vec<Card> = orig_cards.clone();
    let mut i = 0;
    while i < cards.len() {
        let mut j = 0;
        while j < unsafe { cards.get_unchecked(i) }.wins {
            let new_id = cards[i].id + j;
            cards.push(orig_cards[new_id].clone());
            j += 1;
        }

        i += 1;
    }

    println!("Part 2: {0}", cards.len());
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|e| e.to_string()).collect::<Vec<_>>();
    solve_part_one(&lines);
    solve_part_two(&lines);
}
