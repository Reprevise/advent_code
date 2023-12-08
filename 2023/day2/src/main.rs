use std::collections::HashMap;

fn main() {
    let f = std::fs::read_to_string("input.txt").unwrap();
    let lines = f.lines().map(|e| e.to_string()).collect::<Vec<_>>();
    solve_part_one(&lines);
}

struct Game {
    id: i32,
    bags: Vec<HashMap<String, i32>>,
}

impl Game {
    fn new(line: &String, ignore_max: bool) -> Self {
        let mut bags = Vec::<HashMap<String, i32>>::new();

        let (id_part, other) = line.split_once(": ").unwrap();

        // Get game id
        let game_id = id_part
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let bag_groups = other.split("; ");

        for group in bag_groups {
            let mut bags = HashMap::<String, i32>::new();

            for bag in group.split(", ") {
                let (rawCount, name) = bag.split_once(' ').unwrap();
                let count = rawCount.parse::<i32>().unwrap();
            }
        }

        return Self { id: game_id, bags };
    }
}

fn solve_part_one(lines: &Vec<String>) {
    for l in lines {
        let _ = Game::new(l, false);
    }
}
