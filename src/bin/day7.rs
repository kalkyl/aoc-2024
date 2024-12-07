fn main() {
    let equations: Vec<(u64, Vec<u64>)> = std::fs::read_to_string("./input/7.txt")
        .unwrap()
        .lines()
        .filter_map(|l| l.split_once(": "))
        .map(|(value, numbers)| {
            (
                value.parse().unwrap(),
                numbers.split_whitespace().map(|v| v.parse().unwrap()).collect(),
            )
        })
        .collect();

    let sum = sum_valid(&equations, &[u64::wrapping_add, u64::wrapping_mul]);
    println!("Part one: {sum}");

    let sum = sum_valid(&equations, &[u64::wrapping_add, u64::wrapping_mul, concat]);
    println!("Part two: {sum}");
}

fn sum_valid(equations: &[(u64, Vec<u64>)], operators: &[fn(u64, u64) -> u64]) -> u64 {
    equations
        .iter()
        .filter(|(value, numbers)| {
            (0..operators.len().pow((numbers.len() - 1) as u32)).any(|variant| {
                let pattern = pattern(numbers.len() - 1, operators.len(), variant);
                let mut total = numbers[0];
                for i in 0..numbers.len() - 1 {
                    total = operators[pattern[i]](total, numbers[i + 1]);
                }
                total == *value
            })
        })
        .map(|(value, _)| value)
        .sum()
}

fn pattern(len: usize, operators: usize, mut variant: usize) -> Vec<usize> {
    let mut pattern = vec![0; len];
    for p in pattern.iter_mut() {
        *p = variant % operators;
        variant /= operators;
    }
    pattern
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}
