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
                if run(grid).is_err() {
                    count_loops += 1;
                }
            }
        }
    }
    println!("Part two: {count_loops}");
}

fn run(mut grid: Vec<Vec<char>>) -> Result<usize, ()> {
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| r.iter().position(|c| *c == '^').map(|x| (x, y)))
        .unwrap();
    let mut d = [-1, 0, 1, 0];
    let (grid_w, grid_h) = (grid[0].len() as isize, grid.len() as isize);
    let mut trace: std::collections::HashSet<((usize, usize), (isize, isize))> = Default::default();

    grid[y][x] = 'X';
    while (0..grid_h).contains(&(y as isize + d[0])) && (0..grid_w).contains(&(x as isize + d[1])) {
        while grid[(y as isize + d[0]) as usize][(x as isize + d[1]) as usize] == '#' {
            d.rotate_left(1);
        }
        y = (y as isize + d[0]) as usize;
        x = (x as isize + d[1]) as usize;
        grid[y][x] = 'X';
        if !trace.insert(((x, y), (d[1], d[0]))) {
            return Err(());
        }
    }
    Ok(grid.iter().map(|r| r.iter().filter(|c| **c == 'X').count()).sum())
}
