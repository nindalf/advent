use std::fmt::Write;

pub type Point = (usize, usize);

pub struct Grid<T> {
    s: Vec<T>,
    pub rows: usize,
    pub columns: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl<T> Grid<T>
where
    T: Copy + PartialEq,
{
    pub fn construct(input: &str, mapper: impl Fn(char) -> T) -> Grid<T> {
        let columns = if input.is_empty() {
            0
        } else {
            input.lines().next().unwrap().len()
        };
        Grid {
            s: input
                .lines()
                .flat_map(|line| line.chars())
                .map(mapper)
                .collect(),
            rows: input.lines().count(),
            columns,
        }
    }

    pub fn manual_construct(s: Vec<T>, rows: usize, columns: usize) -> Grid<T> {
        Grid { s, rows, columns }
    }

    pub fn get(&self, position: Point) -> Option<T> {
        self.s.get(position.0 * self.columns + position.1).copied()
    }

    pub fn get_i32(&self, position: (i32, i32)) -> Option<T> {
        if position.0 >= self.rows as i32
            || position.0 < 0
            || position.1 >= self.columns as i32
            || position.1 < 0
        {
            return None;
        }
        self.s
            .get(position.0 as usize * self.columns + position.1 as usize)
            .copied()
    }

    pub fn set(&mut self, position: Point, val: T) {
        let idx = position.0 * self.columns + position.1;
        if idx >= self.s.len() {
            return;
        }
        self.s[idx] = val;
    }

    pub fn search(&self, needle: T) -> Option<Point> {
        self.s.iter().enumerate().find_map(|(idx, c)| {
            if *c == needle {
                return Some((idx / self.columns, idx % self.columns));
            }
            None
        })
    }

    pub fn next_position(&self, position: Point, direction: Direction) -> Option<Point> {
        match direction {
            Direction::Up => (position.0 > 0).then(|| (position.0 - 1, position.1)),
            Direction::Right => {
                (position.1 + 1 < self.columns).then(|| (position.0, position.1 + 1))
            }
            Direction::Down => (position.0 + 1 < self.rows).then(|| (position.0 + 1, position.1)),
            Direction::Left => (position.1 > 0).then(|| (position.0, position.1 - 1)),
        }
    }

    pub fn adjacent(&self, position: Point) -> [Option<Point>; 4] {
        let up = (position.0 > 0).then(|| (position.0 - 1, position.1));
        let right = (position.1 + 1 < self.columns).then(|| (position.0, position.1 + 1));
        let down = (position.0 + 1 < self.rows).then(|| (position.0 + 1, position.1));
        let left = (position.1 > 0).then(|| (position.0, position.1 - 1));
        [up, right, down, left]
    }

    pub fn adjacent_one(&self, position: Point) -> [(Option<Point>, Option<T>); 4] {
        let up_one = (position.0 > 0).then(|| (position.0 - 1, position.1));
        let right_one = (position.1 + 1 < self.columns).then(|| (position.0, position.1 + 1));
        let down_one = (position.0 + 1 < self.rows).then(|| (position.0 + 1, position.1));
        let left_one = (position.1 > 0).then(|| (position.0, position.1 - 1));

        let up_one_val = up_one.and_then(|p| self.get(p));
        let right_one_val = right_one.and_then(|p| self.get(p));
        let down_one_val = down_one.and_then(|p| self.get(p));
        let left_one_val = left_one.and_then(|p| self.get(p));

        [
            (up_one, up_one_val),
            (right_one, right_one_val),
            (down_one, down_one_val),
            (left_one, left_one_val),
        ]
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, T)> + use<'_, T> {
        self.s
            .iter()
            .enumerate()
            .map(|(idx, c)| ((idx / self.columns, idx % self.columns), *c))
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        for i in 0..self.rows {
            for j in 0..self.columns {
                let val = self.get((i, j)).unwrap();
                print!("{val}");
            }
            println!();
        }
    }
}

impl Direction {
    pub const ALL_DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };
        f.write_char(c)
    }
}

pub fn euclid_distance(this: &Point, other: &Point) -> usize {
    this.0.abs_diff(other.0) + this.1.abs_diff(other.1)
}
