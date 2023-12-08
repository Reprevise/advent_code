struct Neighbor {
    x: usize,
    y: usize,
    value: char,
}

fn get_neighbors(
    grid: &Vec<String>,
    row_index: usize,
    start_index: usize,
    target_length: usize,
) -> Vec<Neighbor> {
    let mut neighbors = Vec::<Neighbor>::new();
    let end_index = start_index + target_length - 1;
 
    let mut i = row_index.checked_sub(1).unwrap_or(0);
    while i <= (grid.len() - 1).min(row_index + 1) {
        let mut j = start_index.checked_sub(1).unwrap_or(0);
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

fn solve_a(lines: &Vec<String>) {
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
            if neighbors.iter().any(|e| !e.value.is_ascii_digit() && e.value != '.') {
                sum += num.parse::<i32>().unwrap();
            }

            col_idx += num.len();
            num.clear();
        }
    }

    println!("A: {sum}");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|f| f.to_string()).collect::<Vec<String>>();
    solve_a(&lines);
}
