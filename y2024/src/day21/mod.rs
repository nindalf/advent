use ahash::AHashMap;

#[inline]
pub fn part1(input: &str) -> u64 {
    parse(input)
        .map(|keypad| {
            let code = keypad.code;
            let x = "A".to_string() + &keypad.collect::<String>();
            let y = "A".to_string()
                + &x.chars()
                    .map_windows(|[x, y]| next_dpad_movement(*x, *y))
                    .collect::<String>();
            let z = y
                .chars()
                .map_windows(|[x, y]| next_dpad_movement(*x, *y))
                .collect::<String>();
            z.len() as u64 * code
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    parse(input)
        .map(|keypad| {
            let code = keypad.code;
            let x = "A".to_string() + &keypad.collect::<String>();
            let mut result: AHashMap<&str, u64> = x
                .chars()
                .map_windows(|[x, y]| next_dpad_movement(*x, *y))
                .fold(AHashMap::with_capacity(15), |mut acc, movement| {
                    *acc.entry(movement).or_default() += 1;
                    acc
                });
            for _ in 0..26 {
                let mut temp = AHashMap::with_capacity(15);
                for (movement, n) in result {
                    for next_movement in next_dpad_chunk(movement) {
                        if next_movement.is_empty() {
                            continue;
                        }
                        *temp.entry(next_movement).or_default() += n
                    }
                }
                result = temp;
            }
            let sum: u64 = result
                .iter()
                .map(|(movement, n)| (movement.len() as u64) * n)
                .sum();
            sum * code
        })
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = KeyPad> + use<'_> {
    input.lines().filter_map(|line| {
        let code = line[0..3].parse().ok()?;
        let mut chars = line.chars();
        let sequence = [
            NumPadKey::from(chars.next()?),
            NumPadKey::from(chars.next()?),
            NumPadKey::from(chars.next()?),
            NumPadKey::from(chars.next()?),
        ];
        Some(KeyPad {
            current: NumPadKey::Press,
            destination: sequence[0],
            sequence,
            sequence_idx: 0,
            code,
        })
    })
}

struct KeyPad {
    current: NumPadKey,
    destination: NumPadKey,
    sequence: [NumPadKey; 4],
    sequence_idx: usize,
    code: u64,
}

const KEYPAD_GAP: (i32, i32) = (3, 0);

impl Iterator for KeyPad {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sequence_idx >= self.sequence.len() {
            return None;
        }
        self.destination = self.sequence[self.sequence_idx];

        let current = self.current.to_coordinate();
        let destination = self.destination.to_coordinate();

        let vertical = match destination.0 - current.0 {
            -3 => "^^^",
            -2 => "^^",
            -1 => "^",
            0 => "",
            1 => "v",
            2 => "vv",
            3 => "vvv",
            _ => unreachable!(),
        };
        let horizontal = match destination.1 - current.1 {
            -2 => "<<",
            -1 => "<",
            0 => "",
            1 => ">",
            2 => ">>",
            _ => unreachable!(),
        };
        let result = if destination.1 > current.1 && (destination.0, current.1) != KEYPAD_GAP {
            format!("{vertical}{horizontal}A")
        } else if (current.0, destination.1) != KEYPAD_GAP {
            format!("{horizontal}{vertical}A")
        } else {
            format!("{vertical}{horizontal}A")
        };

        self.current = self.destination;
        self.sequence_idx += 1;

        Some(result)
    }
}

fn next_dpad_movement(current: char, destination: char) -> &'static str {
    match (current, destination) {
        ('^', '^') => "A",
        ('^', '>') => ">vA",
        ('^', '<') => "v<A",
        ('^', 'A') => ">A",
        ('>', '^') => "<^A",
        ('>', '>') => "A",
        ('>', 'v') => "<A",
        ('>', 'A') => "^A",
        ('v', '>') => ">A",
        ('v', 'v') => "A",
        ('v', '<') => "<A",
        ('v', 'A') => ">^A",
        ('<', '^') => ">^A",
        ('<', 'v') => ">A",
        ('<', '<') => "A",
        ('<', 'A') => ">>^A",
        ('A', '^') => "<A",
        ('A', '>') => "vA",
        ('A', 'v') => "<vA",
        ('A', '<') => "v<<A",
        ('A', 'A') => "A",
        // ('^', 'v') => "vA", // Should never happen
        // ('>', '<') => "<<A", // Should never happen
        // ('v', '^') => "^A", // Should never happen
        // ('<', '>') => ">>A", // Should never happen
        _ => unreachable!("All cases handled"),
    }
}

fn next_dpad_chunk(movements: &str) -> [&'static str; 4] {
    match movements {
        "^A" => ["<A", ">A", "", ""],
        "<^A" => ["v<<A", ">^A", ">A", ""],
        "<A" => ["v<<A", ">>^A", "", ""],
        "<vA" => ["v<<A", ">A", ">^A", ""],
        ">^A" => ["vA", "<^A", ">A", ""],
        ">>^A" => ["vA", "A", "<^A", ">A"],
        ">A" => ["vA", "^A", "", ""],
        ">vA" => ["vA", "<A", ">^A", ""],
        "A" => ["A", "", "", ""],
        "v<<A" => ["<vA", "<A", "A", ">>^A"],
        "v<A" => ["<vA", "<A", ">>^A", ""],
        "vA" => ["<vA", ">^A", "", ""],
        _ => unreachable!("Unrecognised movement"),
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum NumPadKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Press,
}

impl From<char> for NumPadKey {
    fn from(value: char) -> Self {
        use NumPadKey::{Eight, Five, Four, Nine, One, Press, Seven, Six, Three, Two, Zero};
        match value {
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            '0' => Zero,
            'A' => Press,
            _ => unreachable!("invalid input"),
        }
    }
}

impl NumPadKey {
    fn to_coordinate(self) -> (i32, i32) {
        match self {
            NumPadKey::Zero => (3, 1),
            NumPadKey::One => (2, 0),
            NumPadKey::Two => (2, 1),
            NumPadKey::Three => (2, 2),
            NumPadKey::Four => (1, 0),
            NumPadKey::Five => (1, 1),
            NumPadKey::Six => (1, 2),
            NumPadKey::Seven => (0, 0),
            NumPadKey::Eight => (0, 1),
            NumPadKey::Nine => (0, 2),
            NumPadKey::Press => (3, 2),
        }
    }
}

common::aoctest!(126384, 188398, 1234, 230049027535970);
