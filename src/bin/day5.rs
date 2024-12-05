fn main() {
    let input = std::fs::read_to_string("./input/5.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let sections: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let rules: Vec<(u32, u32)> = sections[0]
        .iter()
        .filter_map(|s| s.split_once('|').map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())))
        .collect();
    let mut updates: Vec<Vec<u32>> = sections[1]
        .iter()
        .filter_map(|s| s.split(',').map(|x| x.parse().ok()).collect())
        .collect();

    let sum: u32 = updates
        .iter()
        .filter(|pages| pages.is_sorted_by(|a, b| rules.contains(&(*a, *b))))
        .map(|pages| pages[pages.len() / 2])
        .sum();
    println!("Part one: {sum}");

    let sum: u32 = updates
        .iter_mut()
        .filter(|pages| !pages.is_sorted_by(|a, b| rules.contains(&(*a, *b))))
        .map(|pages| {
            pages.sort_by(|a, b| rules.contains(&(*a, *b)).cmp(&true));
            pages[pages.len() / 2]
        })
        .sum();
    println!("Part two: {sum}");
}
