#[inline]
pub fn part1(input: &str) -> u32 {
    let grid = parse(input);
    let mut results = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.columns() {
            let i = i as i32;
            let j = j as i32;
            if grid.get(i, j) == Some('X') {
                for u in -1..=1 {
                    for v in -1..=1 {
                        if u == 0 && v == 0 {
                            continue;
                        }

                        if let (Some('M'), Some('A'), Some('S')) = (
                            grid.get(i + u, j + v),
                            grid.get(i + 2 * u, j + 2 * v),
                            grid.get(i + 3 * u, j + 3 * v),
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
    for i in 0..grid.rows() {
        for j in 0..grid.columns() {
            let i = i as i32;
            let j = j as i32;
            if grid.get(i, j) == Some('A') {
                match (
                    grid.get(i - 1, j - 1),
                    grid.get(i + 1, j + 1),
                    grid.get(i + 1, j - 1),
                    grid.get(i - 1, j + 1),
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

fn parse(input: &str) -> Grid {
    Grid {
        s: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

struct Grid {
    s: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, i: i32, j: i32) -> Option<char> {
        if i < 0 || j < 0 {
            return None;
        }
        let i = i as usize;
        let j = j as usize;
        match self.s.get(i) {
            Some(v) => v.get(j).copied(),
            None => None,
        }
    }

    fn rows(&self) -> usize {
        self.s.len()
    }

    fn columns(&self) -> usize {
        if self.s.is_empty() {
            return 0;
        }
        self.s[0].len()
    }
}

crate::aoctest!(18, 2560, 9, 1910);
