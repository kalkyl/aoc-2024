use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/3.txt")?;

    let sum: i32 = muls(&input).iter().map(|(a, b)| a * b).sum();
    println!("Part one: {sum}");

    let dos: String = input
        .split("do()")
        .filter_map(|s| s.split_once("don't()").map(|(l, _r)| l).or(Some(s)))
        .collect();
    let sum: i32 = muls(&dos).iter().map(|(a, b)| a * b).sum();
    println!("Part two: {sum}");

    Ok(())
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
