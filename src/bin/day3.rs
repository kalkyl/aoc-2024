fn main() {
    let input = std::fs::read_to_string("./input/3.txt").unwrap();

    let sum: i32 = muls(&input).iter().map(|(a, b)| a * b).sum();
    println!("Part one: {sum}");

    let dos: String = input.split("do()").filter_map(|s| s.split("don't()").next()).collect();
    let sum: i32 = muls(&dos).iter().map(|(a, b)| a * b).sum();
    println!("Part two: {sum}");
}

fn muls(input: &str) -> Vec<(i32, i32)> {
    input
        .split("mul(")
        .filter_map(|s| {
            s.split_once(')').and_then(|(l, _r)| {
                l.split_once(',').and_then(|(a, b)| match (a.parse(), b.parse()) {
                    (Ok(a), Ok(b)) => Some((a, b)),
                    _ => None,
                })
            })
        })
        .collect()
}
