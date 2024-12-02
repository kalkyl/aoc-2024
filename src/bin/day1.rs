use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let lines: Vec<(u32, u32)> = read_to_string("./input/1.txt")?
        .lines()
        .map(|s| s.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let mut list_l: Vec<_> = lines.iter().map(|&(l, _r)| l).collect();
    list_l.sort();

    let mut list_r: Vec<_> = lines.iter().map(|&(_l, r)| r).collect();
    list_r.sort();

    let diff_sum: u32 = list_l.iter().zip(list_r.iter()).map(|(l, r)| l.abs_diff(*r)).sum();
    println!("Part one: {:?}", diff_sum);

    let sim_score: u32 = list_l
        .iter()
        .map(|l| l * list_r.iter().filter(|r| *r == l).count() as u32)
        .sum();
    println!("Part two: {:?}", sim_score);

    Ok(())
}
