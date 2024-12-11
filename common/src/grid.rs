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

    pub fn iter(&self) -> impl Iterator<Item = (Point, T)> + use<'_, T> {
        self.s
            .iter()
            .enumerate()
            .map(|(idx, c)| ((idx / self.columns, idx % self.columns), *c))
    }
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
