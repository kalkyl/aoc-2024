use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let reports: Vec<Vec<i32>> = read_to_string("./input/2.txt")?
        .lines()
        .map(|s| s.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let safe_count = reports.iter().filter(|r| is_safe(r)).count();
    println!("Part one: {:?}", safe_count);

    let safe_count = reports
        .iter()
        .filter(|r| {
            is_safe(r)
                || (0..r.len()).any(|i| {
                    let mut report = r.to_vec();
                    report.remove(i);
                    is_safe(&report)
                })
        })
        .count();
    println!("Part two: {:?}", safe_count);

    Ok(())
}

fn is_safe(r: &[i32]) -> bool {
    (0..r.len() - 1)
        .all(|i| (r[i + 1] - r[i]).signum() == (r[1] - r[0]).signum() && (1..=3).contains(&r[i].abs_diff(r[i + 1])))
}
