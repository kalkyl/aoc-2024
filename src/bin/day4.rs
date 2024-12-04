use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/4.txt")?;
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let word: Vec<_> = "XMAS".chars().collect();
    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            //Part 1
            if grid[y][x] == word[0] {
                for dy in -1..=1_i32 {
                    for dx in -1..=1_i32 {
                        let mut found = true;
                        for (i, c) in word.iter().enumerate().skip(1) {
                            let cx = x as i32 + dx * i as i32;
                            let cy = y as i32 + dy * i as i32;
                            if !(0..grid[y].len() as i32).contains(&cx)
                                || !(0..grid.len() as i32).contains(&cy)
                                || grid[cy as usize][cx as usize] != *c
                            {
                                found = false;
                                break;
                            }
                        }
                        if found {
                            count_part1 += 1;
                        }
                    }
                }
            }

            // Part 2
            if grid[y][x] == 'A' && (1..grid[y].len() - 1).contains(&x) && (1..grid.len() - 1).contains(&y) {
                let d1 = String::from_iter([grid[y - 1][x - 1], 'A', grid[y + 1][x + 1]]);
                let d2 = String::from_iter([grid[y + 1][x - 1], 'A', grid[y - 1][x + 1]]);
                if (d1 == "MAS" || d1 == "SAM") && (d2 == "MAS" || d2 == "SAM") {
                    count_part2 += 1
                }
            }
        }
    }
    println!("Part one: {:?}", count_part1);
    println!("Part two: {:?}", count_part2);

    Ok(())
}
