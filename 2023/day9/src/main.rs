fn find_next(mut nums: Vec<Vec<isize>>) -> (isize, isize) {
    nums.reverse();
    let mut start: isize = 0;
    let mut end: isize = 0;

    for n in nums.iter().skip(1) {
        let first = *n.first().unwrap();
        if start == 0 {
            start = first;
        } else {
            start = first - start;
        }

        let last = *n.last().unwrap();
        if end == 0 {
            end = last;
        } else {
            end += last;
        }
    }

    (start, end)
}

fn drilldown(nums: Vec<isize>) -> Vec<Vec<isize>> {
    let mut copy = vec![nums];
    let mut idx: usize = 0;

    loop {
        let mut new_line: Vec<isize> = vec![];
        for pair in copy.get(idx).unwrap().windows(2) {
            let [a, b] = pair else {
                unreachable!();
            };
            new_line.push(b - a);
        }
        copy.push(new_line.clone());
        if new_line.iter().all(|v| v == &0) {
            break;
        }
        idx += 1;
    }

    return copy;
}

fn solve_part_one(lines: &[&str]) {
    let mut sum: isize = 0;

    for l in lines {
        let nums = l
            .split_ascii_whitespace()
            .map(|e| e.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let nums = drilldown(nums);
        let (_, res) = find_next(nums);
        sum += res;
    }

    println!("Part 1: {sum}");
}

fn solve_part_two(lines: &[&str]) {
    let mut sum: isize = 0;

    for l in lines {
        let nums = l
            .split_ascii_whitespace()
            .map(|e| e.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let nums = drilldown(nums);
        let (res, _) = find_next(nums);
        sum += res;
    }

    println!("Part 2: {sum}");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    solve_part_one(&lines);
    solve_part_two(&lines);
}
