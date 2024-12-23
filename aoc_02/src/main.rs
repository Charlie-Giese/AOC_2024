fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn assert_safety(level: String) -> bool {
    let values = level
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("{:?}", values);

    let reversed = values.iter().copied().rev().collect::<Vec<i32>>();

    let safety = (values.is_sorted() || reversed.is_sorted())
        && (values
            .windows(2)
            .all(|w| (w[0] - w[1]).abs() < 4 && w[0] != w[1]));

    safety
}

fn main() {
    let lines = read_lines("input.txt");

    let mut count = 0;

    for level in lines.iter() {
        if assert_safety(level.clone()) {
            count += 1
        };
    }

    println!("{:?}", count);
}
