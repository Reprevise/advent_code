use std::iter::zip;

fn get_num_list(line: &str) -> impl Iterator<Item = usize> + '_ {
    let raw_list = line.split_once(": ").unwrap();
    let nums = raw_list
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    nums
}

fn solve_part_one(lines: &Vec<&str>) {
    let times = get_num_list(lines.iter().nth(0).unwrap());
    let distances = get_num_list(lines.iter().nth(1).unwrap());
    let pairs = zip(times, distances);

    let mut n: Vec<usize> = vec![];

    for (time, distance) in pairs {
        let mut t = 0;
        for x in 1..=time {
            let mm_traveled = (time - x) * x;
            if mm_traveled > distance {
                t += 1;
            }
        }
        n.push(t);
    }

    println!("Part 1: {0}", n.iter().product::<usize>());
}

fn solve_part_two(lines: &Vec<&str>) {
    let mut iter = lines.iter();
    let time = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .fold(String::new(), |agg, x| agg + x)
        .parse::<usize>()
        .unwrap();
    let distance = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .fold(String::new(), |agg, x| agg + x)
        .parse::<usize>()
        .unwrap();

    let mut t = 0;
    for x in 1..=time {
        let mm_traveled = (time - x) * x;
        if mm_traveled > distance {
            t += 1;
        }
    }

    println!("Part 2: {0}", &t);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    solve_part_one(&lines);
    solve_part_two(&lines);
}
