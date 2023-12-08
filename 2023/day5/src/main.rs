#[derive(Debug)]
struct ProblemMap {
    ranges: Vec<(usize, usize, usize)>,
}

impl ProblemMap {
    fn run(&self, seeds: &Vec<usize>) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];

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

    fn run_range(&self, seed_ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let mut a: Vec<(usize, usize)> = vec![];
        let mut ranges_clone = seed_ranges.to_owned();

        for (dest, src, sz) in &self.ranges {
            let src_end = src + sz;
            let mut new_ranges: Vec<(usize, usize)> = vec![];
            for (st, ed) in &ranges_clone {
                let before = (*st, *ed.min(src));
                let inter = (*st.max(src), src_end.min(*ed));
                let after = (src_end.max(*st), *ed);
                if before.1 > before.0 {
                    new_ranges.push(before);
                }
                if inter.1 > inter.0 {
                    a.push((inter.0 - src + dest, inter.1 - src + dest));
                }
                if after.1 > after.0 {
                    new_ranges.push(after);
                }
            }
            ranges_clone = new_ranges;
        }

        [a, ranges_clone].concat()
    }
}

fn parse_map(map_str: &str) -> ProblemMap {
    let lines = map_str.lines().skip(1);

    let mut ranges: Vec<(usize, usize, usize)> = vec![];
    for l in lines {
        let mut nums = l
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let dr = nums.next().unwrap();
        let sr = nums.next().unwrap();
        let rl = nums.next().unwrap();

        ranges.push((dr, sr, rl));
    }

    ProblemMap { ranges }
}

fn solve_part_one(seeds: &[usize], maps: &Vec<ProblemMap>) {
    let mut start = seeds.to_owned();
    for m in maps {
        start = m.run(&start);
    }

    let lowest = start.iter().min().unwrap();

    println!("Part One: {lowest}");
}

fn solve_part_two(seed_ranges: &[usize], maps: &Vec<ProblemMap>) {
    let seed_ranges = seed_ranges
        .chunks_exact(2)
        .map(|c| match c {
            [start, length] => (*start, *length),
            _ => unreachable!(),
        });

    let mut p2: Vec<usize> = vec![];
    for (st, sz) in seed_ranges {
        let mut start = vec![(st, st + sz)];
        for m in maps {
            start = m.run_range(&start);
        }
        p2.push(start.iter().map(|a| a.0).min().unwrap())
    }

    let lowest = p2.iter().min().unwrap();

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
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let maps = groups.map(parse_map).collect::<Vec<_>>();

    solve_part_one(&seeds, &maps);
    solve_part_two(&seeds, &maps);
}
