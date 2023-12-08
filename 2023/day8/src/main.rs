use std::collections::HashMap;

fn parse_input(lines: std::slice::Iter<&str>) -> HashMap<String, (String, String)> {
    let mut result = HashMap::<String, (String, String)>::new();

    for l in lines {
        let s = l.split_once(" = ").unwrap();
        let key = s.0.to_owned();
        let pair_split = s.1.split_once(", ").unwrap();
        let left = pair_split.0.chars().skip(1).collect::<String>();
        let right = pair_split.1.chars().take(3).collect::<String>();

        result.insert(key, (left, right));
    }

    result
}

fn solve_part_one(lines: &[&str]) {
    let mut iter = lines.iter();
    let instructions = iter.next().unwrap().chars().collect::<Vec<_>>();
    let _ = iter.next().unwrap();
    let parsed = parse_input(iter);

    let mut instr_idx: usize = 0;
    let mut steps: usize = 0;
    let mut start = "AAA";

    while let Some(c) = instructions.get(instr_idx % instructions.len()) {
        let (l, r) = parsed.get(start).unwrap();

        match c {
            'L' => {
                start = l;
            }
            'R' => {
                start = r;
            }
            _ => unreachable!(),
        }

        steps += 1;

        if start == "ZZZ" {
            break;
        }

        instr_idx += 1;
    }

    println!("Part 1: {steps}");
}

fn lcm_of_iter<'a>(numbers: impl Iterator<Item = &'a usize>) -> usize {
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    numbers.fold(1, |acc, num| acc / gcd(acc, *num) * num)
}

fn solve_part_two(lines: &[&str]) {
    let mut iter = lines.iter();
    let instructions = iter.next().unwrap().chars().collect::<Vec<_>>();
    let _ = iter.next().unwrap();
    let parsed = parse_input(iter);

    let mut steps: usize = 0;
    let mut start = parsed
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let result: usize;
    let mut steps_per = HashMap::<usize, usize>::new();
    'outer: loop {
        let mut new_nodes: Vec<&String> = vec![];

        let instr_char = instructions.get(steps % instructions.len()).unwrap();

        for (i, p) in start.iter().enumerate() {
            let (l, r) = parsed.get(*p).unwrap();
            let node = match instr_char {
                'L' => l,
                'R' => r,
                _ => unreachable!(),
            };
            if node.ends_with('Z') {
                steps_per.insert(i, steps + 1);

                if steps_per.len() == start.len() {
                    result = lcm_of_iter(steps_per.values());

                    break 'outer;
                }
            }
            new_nodes.push(node);
        }

        start = new_nodes;
        steps += 1;
    }

    println!("Part 2 {0}", result);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    solve_part_one(&lines);
    solve_part_two(&lines);
}
