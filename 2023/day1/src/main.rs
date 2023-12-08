fn word_to_number(word: &'static str) -> Option<u32> {
    return match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };
}

const WORDS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn line_to_numbers(line: &str) -> Vec<u32> {
    let mut buffer = String::new();
    let mut result = Vec::<u32>::new();

    for c in line.chars() {
        buffer.push(c);

        if buffer.len() >= 3 {
            let buffer_slice = buffer.clone().as_str().clone();

            if WORDS.contains(&buffer_slice.clone()) {
                if let Some(number) = word_to_number(buffer_slice.clone()) {
                    result.push(number);
                    buffer.clear();
                }
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let by_line = input.lines();

    let mut sum: u32 = 0;

    for line in by_line {
        let mut buffer = String::with_capacity(line.len());

        let mut num: String = String::with_capacity(2);
        let mut last_digit_char: Option<char> = None;

        let words = line.chars().fold(Vec::<&str>::new(), |agg, v| {
            println!("{v}");

            return agg;
        });

        for c in line.chars() {
            buffer.push(c);

            if c.is_ascii_digit() {
                if num.is_empty() {
                    num.push(c);
                }

                last_digit_char = Some(c);
            } else if buffer.len() >= 3 {
            }
        }

        num.push(last_digit_char.unwrap());

        sum += u32::from_str_radix(num.as_str(), 10).unwrap();
    }

    println!("Sum {sum}");

    return Ok(());
}
