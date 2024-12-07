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

    let sum = sum_valid(&equations, &[add, multiply]);
    println!("Part one: {sum}");

    let sum = sum_valid(&equations, &[add, multiply, concat]);
    println!("Part two: {sum}");
}

fn sum_valid(equations: &[(u64, Vec<u64>)], operators: &[fn(u64, u64) -> u64]) -> u64 {
    let mut sum = 0;
    for (value, numbers) in equations {
        if (0..(operators.len() as u64).pow((numbers.len() - 1) as u32)).any(|combination| {
            let mut total = numbers[0];
            for i in 0..numbers.len() - 1 {
                let operator = *to_base(combination as usize, operators.len()).get(i).unwrap_or(&0);
                total = operators[operator as usize](total, numbers[i + 1]);
            }
            total == *value
        }) {
            sum += value;
        }
    }
    sum
}

fn to_base(mut num: usize, base: usize) -> Vec<usize> {
    let mut positions = Vec::new();
    while num > 0 {
        positions.push(num % base);
        num /= base;
    }
    positions
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}