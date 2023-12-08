use std::collections::HashSet;

struct Neighbor {
    x: usize,
    y: usize,
    value: char,
}

fn get_neighbors(
    grid: &[String],
    row_index: usize,
    start_index: usize,
    target_length: usize,
) -> Vec<Neighbor> {
    let mut neighbors = Vec::<Neighbor>::new();
    let end_index = start_index + target_length - 1;

    let mut i = row_index.saturating_sub(1);
    while i <= (grid.len() - 1).min(row_index + 1) {
        let mut j = start_index.saturating_sub(1);
        while j <= (grid[i].len() - 1).min(end_index + 1) {
            if i == row_index && j >= start_index && j <= end_index {
                j += 1;
                continue;
            }

            neighbors.push(Neighbor {
                x: i,
                y: j,
                value: grid[i].chars().nth(j).unwrap(),
            });

            j += 1;
        }

        i += 1;
    }

    neighbors
}

fn solve_part_one(lines: &[String]) {
    let mut sum = 0;

    for (row_idx, row) in lines.iter().enumerate() {
        let mut num = String::new();

        let mut col_idx = 0;
        while col_idx < row.len() {
            let col = row.chars().nth(col_idx).unwrap();
            if !col.is_ascii_digit() {
                col_idx += 1;
                continue;
            }

            num.push(col);

            let mut next = row.chars().nth(col_idx + 1);
            while next.is_some_and(|n| n.is_ascii_digit()) {
                num.push(next.unwrap());
                next = row.chars().nth(col_idx + num.len());
            }

            let neighbors = get_neighbors(lines, row_idx, col_idx, num.len());
            if neighbors
                .iter()
                .any(|e| !e.value.is_ascii_digit() && e.value != '.')
            {
                sum += num.parse::<i32>().unwrap();
            }

            col_idx += num.len();
            num.clear();
        }
    }

    println!("Part 1: {sum}");
}

// B

fn extract_number(grid: &[String], x: usize, y: usize) -> usize {
    let mut num = String::new();

    // println!("{:?}", grid[x].chars().skip(y).collect::<Vec<char>>());

    for c in grid[x].chars().skip(y) {
        if !c.is_ascii_digit() {
            break;
        }
        num.push(c);
    }

    num.parse::<usize>().unwrap()
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn solve_part_two(lines: &[String]) {
    let mut sum = 0;

    let mut i: usize = 0;
    while i < lines.len() {
        let mut j: usize = 0;
        while j < lines[i].len() {
            if lines[i].chars().nth(j).unwrap() == '*' {
                let mut adj: HashSet<usize> = HashSet::new();

                for (x, y) in DIRS {
                    let nx = (i as isize) + x;
                    let ny = (j as isize) + y;
                    if nx >= 0
                        && (nx as usize) < lines.len()
                        && ny >= 0
                        && (ny as usize) < lines[nx as usize].len()
                        && lines[nx as usize]
                            .chars()
                            .nth(ny as usize)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        let mut start = ny as usize;
                        while start > 0
                            && lines[nx as usize]
                                .chars()
                                .nth(start - 1)
                                .unwrap()
                                .is_ascii_digit()
                        {
                            start -= 1;
                        }
                        let num = extract_number(lines, nx as usize, start);
                        if num > 0 {
                            adj.insert(num);
                        }
                    }
                }

                if adj.len() == 2 {
                    sum += adj.iter().product::<usize>();
                }
            }

            j += 1;
        }

        i += 1;
    }

    println!("Part 2: {sum}");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input
        .lines()
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    solve_part_one(&lines);
    solve_part_two(&lines);
}
