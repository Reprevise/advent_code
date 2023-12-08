#[derive(Debug)]
struct ProblemMap {
    ranges: Vec<(u32, u32, u32)>,
}

impl ProblemMap {
    fn run(&self, seeds: &Vec<u32>) -> Vec<u32> {
        let mut result: Vec<u32> = vec![];

        'outer: for seed in seeds {
            for (dr, sr, rl) in &self.ranges {
                let (start, end) = (sr, sr + (rl - 1));
                if seed >= start && seed <= &end {
                    let diff = seed - start;
                    result.push(dr + diff);
                    continue 'outer;
                }
            }

            result.push(*seed);
        }

        result
    }
}

fn parse_map(map_str: &str) -> ProblemMap {
    let lines = map_str.lines().skip(1);

    let mut ranges: Vec<(u32, u32, u32)> = vec![];
    for l in lines {
        let mut nums = l
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let dr = nums.next().unwrap();
        let sr = nums.next().unwrap();
        let rl = nums.next().unwrap();

        ranges.push((dr, sr, rl));
    }

    ProblemMap {
        ranges,
    }
}

fn solve_part_one(seeds: &Vec<u32>, maps: &Vec<ProblemMap>) {
    let mut start = seeds.clone();
    for m in maps {
        start = m.run(&start);
    }

    let lowest = start.iter().min().unwrap();

    println!("Part One: {lowest}");
}

fn solve_part_two(seed_ranges: &Vec<u32>, maps: &Vec<ProblemMap>) {
    let mut seeds: Vec<u32> = vec![];

    for x in seed_ranges.chunks_exact(2) {
        println!("{:?}", x);
        let [start, length] = x else {
            unreachable!();
        };

        for j in *start..(start + length) {
            seeds.push(j);
        }
    }

    println!("{0}", seeds.len());

    let mut start = seeds;
    for m in maps {
        start = m.run(&start);
    }

    let lowest = start.iter().min().unwrap();

    println!("Part Two: {lowest}");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut groups = input.split("\r\n\r\n");
    let seeds = groups
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let maps = groups.map(parse_map).collect::<Vec<_>>();

    solve_part_one(&seeds, &maps);
    // solve_part_two(&seeds, &maps);
}
