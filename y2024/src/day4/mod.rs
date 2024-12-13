use common::grid::Grid;

#[inline]
pub fn part1(input: &str) -> u32 {
    let grid = parse(input);
    let mut results = 0;
    for i in 0..grid.rows as i32 {
        for j in 0..grid.columns as i32 {
            if grid.get_i32((i, j)) == Some('X') {
                for u in -1..=1 {
                    for v in -1..=1 {
                        if u == 0 && v == 0 {
                            continue;
                        }

                        if let (Some('M'), Some('A'), Some('S')) = (
                            grid.get_i32((i + u, j + v)),
                            grid.get_i32((i + 2 * u, j + 2 * v)),
                            grid.get_i32((i + 3 * u, j + 3 * v)),
                        ) {
                            results += 1
                        }
                    }
                }
            }
        }
    }
    results
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let grid = parse(input);
    let mut results = 0;
    for i in 1..grid.rows - 1 {
        for j in 1..grid.columns - 1 {
            if grid.get((i, j)) == Some('A') {
                match (
                    grid.get((i - 1, j - 1)),
                    grid.get((i + 1, j + 1)),
                    grid.get((i + 1, j - 1)),
                    grid.get((i - 1, j + 1)),
                ) {
                    (Some('M'), Some('S'), Some('M'), Some('S')) => results += 1,
                    (Some('S'), Some('M'), Some('M'), Some('S')) => results += 1,
                    (Some('M'), Some('S'), Some('S'), Some('M')) => results += 1,
                    (Some('S'), Some('M'), Some('S'), Some('M')) => results += 1,
                    _ => (),
                }
            }
        }
    }
    results
}

pub fn parse(input: &str) -> Grid<char> {
    Grid::construct(input, |x| x)
}

common::aoctest!(18, 2560, 9, 1910);
