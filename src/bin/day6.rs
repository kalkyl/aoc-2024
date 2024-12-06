fn main() {
    let input = std::fs::read_to_string("./input/6.txt").unwrap();
    let start_grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let count = run(start_grid.clone()).unwrap();
    println!("Part one: {count}");

    let mut count_loops = 0;
    for y in 0..start_grid.len() {
        for x in 0..start_grid[y].len() {
            if start_grid[y][x] == '.' {
                let mut grid = start_grid.clone();
                grid[y][x] = '#';
                if run(grid).is_none() {
                    count_loops += 1;
                }
            }
        }
    }
    println!("Part two: {count_loops}");
}

fn run(mut grid: Vec<Vec<char>>) -> Option<usize> {
    let mut trace: std::collections::HashSet<((isize, isize), (isize, isize))> = Default::default();
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| r.iter().position(|c| *c == '^').map(|x| (x as isize, y as isize)))
        .unwrap();
    let (w, h) = (grid[0].len() as isize, grid.len() as isize);
    let mut d = [-1, 0, 1, 0];

    grid[y as usize][x as usize] = 'X';
    while (0..h).contains(&(y + d[0])) && (0..w).contains(&(x + d[1])) {
        while grid[(y + d[0]) as usize][(x + d[1]) as usize] == '#' {
            d.rotate_left(1);
        }
        y += d[0];
        x += d[1];
        grid[y as usize][x as usize] = 'X';
        if !trace.insert(((x, y), (d[1], d[0]))) {
            return None;
        }
    }
    Some(grid.iter().map(|r| r.iter().filter(|c| **c == 'X').count()).sum())
}
