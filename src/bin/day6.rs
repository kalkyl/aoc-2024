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
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| r.iter().position(|c| *c == '^').map(|x| (x, y)))
        .unwrap();
    let mut rx = [0, 1, 0, -1].into_iter().cycle();
    let mut ry = [-1, 0, 1, 0].into_iter().cycle();
    let (mut dx, mut dy) = (rx.next().unwrap(), ry.next().unwrap());
    let (grid_w, grid_h) = (grid[0].len() as isize, grid.len() as isize);
    let mut trace: std::collections::HashSet<((usize, usize), (isize, isize))> = Default::default();

    grid[y][x] = 'X';
    while (0..grid_h).contains(&(y as isize + dy)) && (0..grid_w).contains(&(x as isize + dx)) {
        while grid[(y as isize + dy) as usize][(x as isize + dx) as usize] == '#' {
            dx = rx.next().unwrap();
            dy = ry.next().unwrap();
        }
        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
        grid[y][x] = 'X';
        if !trace.insert(((x, y), (dx, dy))) {
            return None;
        }
    }
    Some(grid.iter().map(|r| r.iter().filter(|c| **c == 'X').count()).sum())
}
