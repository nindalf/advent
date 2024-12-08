pub struct Grid {
    s: Vec<char>,
    pub rows: usize,
    pub columns: usize,
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let columns = if input.is_empty() {
            0
        } else {
            input.lines().next().unwrap().len()
        };
        Grid {
            s: input.lines().flat_map(|line| line.chars()).collect(),
            rows: input.lines().count(),
            columns,
        }
    }

    pub fn get(&self, position: (usize, usize)) -> Option<char> {
        self.s.get(position.0 * self.columns + position.1).copied()
    }

    pub fn set(&mut self, position: (usize, usize), val: char) {
        let idx = position.0 * self.columns + position.1;
        if idx >= self.s.len() {
            return ();
        }
        self.s[idx] = val;
    }

    pub fn search(&self, needle: char) -> Option<(usize, usize)> {
        self.s.iter().enumerate().find_map(|(idx, c)| {
            if *c == needle {
                return Some((idx / self.columns, idx % self.columns));
            }
            None
        })
    }
}
