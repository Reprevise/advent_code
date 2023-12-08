use std::collections::HashMap;

fn main() {
    let f = std::fs::read_to_string("input.txt").unwrap();
    let lines = f.lines().collect::<Vec<_>>();
    solve_part_one(&lines);
    solve_part_two(&lines);
}

struct Game<'a> {
    id: usize,
    bags: Vec<HashMap<&'a str, usize>>,
}

const RED_MAX: usize = 12;
const GREEN_MAX: usize = 13;
const BLUE_MAX: usize = 14;

fn is_at_max(name: &str, amount: usize) -> bool {
    match name {
        "red" => amount > RED_MAX,
        "green" => amount > GREEN_MAX,
        "blue" => amount > BLUE_MAX,
        _ => unreachable!(),
    }
}

impl<'a> Game<'a> {
    fn new(line: &'a str, ignore_max: bool) -> Option<Self> {
        let mut all_bags = Vec::<HashMap<&str, usize>>::new();

        let (id_part, other) = line.split_once(": ").unwrap();

        // Get game id
        let game_id = id_part
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let bag_groups = other.split("; ");

        for group in bag_groups {
            let mut bags = HashMap::<&str, usize>::new();

            for bag in group.split(", ") {
                let (raw_count, name) = bag.split_once(' ').unwrap();
                let count = raw_count.parse::<usize>().unwrap();

                if !ignore_max && is_at_max(name, count) {
                    return None;
                }

                bags.insert(name, count);
            }

            all_bags.push(bags);
        }

        return Some(Self {
            id: game_id,
            bags: all_bags,
        });
    }
}

fn solve_part_one(lines: &[&str]) {
    let mut sum: usize = 0;

    for l in lines {
        let Some(game) = Game::new(l, false) else {
            continue;
        };
        sum += game.id;
    }

    println!("Part 1: {sum}");
}

fn solve_part_two(lines: &[&str]) {
    let mut sum: usize = 0;

    for l in lines {
        let game = Game::new(l, true).unwrap();

        let mut mins: HashMap<&str, usize> = HashMap::new();

        for group in game.bags {
            for (key, value) in group.iter() {
                mins.entry(key).and_modify(|v| *v = *value.max(v)).or_insert(*value);
            }
        }

        sum += mins.values().product::<usize>();
    }

    println!("Part 2: {sum}");
}
