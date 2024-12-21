#[inline]
pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|(dpad, code)| {
            let x = dpad.map(|dpadkey| dpadkey.to_char()).collect::<String>();
            println!("{code}* {} {:?}", x.len(), x);
            x.len() as u32 * code
        })
        .sum()
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    0
}

fn parse(input: &str) -> impl Iterator<Item = (Box<DPad>, u32)> + use<'_> {
    input
        .lines()
        .filter_map(|line| {
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
                current_direction: DPadKey::Press,
                sequence,
                sequence_idx: 0,
                code,
                all_keys: Vec::new(),
            })
        })
        .map(|keypad| {
            let code = keypad.code;
            let mut boxed_keypad = Box::new(keypad);
            let next = boxed_keypad.next().unwrap();
            let mut dpad_one = Box::new(DPad {
                remote: boxed_keypad,
                current: DPadKey::Press,
                destination: next,
                current_direction: DPadKey::Press,
                all_keys: Vec::new(),
            });
            let next = dpad_one.next().unwrap();
            let dpad_two = Box::new(DPad {
                remote: dpad_one,
                current: DPadKey::Press,
                destination: next,
                current_direction: DPadKey::Press,
                all_keys: Vec::new(),
            });
            (dpad_two, code)
        })
}

struct KeyPad {
    current: NumPadKey,
    destination: NumPadKey,
    current_direction: DPadKey,
    sequence: [NumPadKey; 4],
    sequence_idx: usize,
    code: u32,
    all_keys: Vec<DPadKey>,
}

impl Iterator for KeyPad {
    type Item = DPadKey;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.destination {
            if self.sequence_idx == self.sequence.len() && self.current_direction == DPadKey::Press
            {
                // println!("{}", self.all_keys.iter().map(|dpadkey| dpadkey.to_char()).collect::<String>());
                return None;
            }
            self.sequence_idx += 1;
            if self.sequence_idx < self.sequence.len() {
                self.destination = self.sequence[self.sequence_idx];
            }
            self.current_direction = DPadKey::Press;
            // println!(
            //     "NumPad Reached destination {:?}, changing destination to {:?}",
            //     self.current, self.destination
            // );
            self.all_keys.push(DPadKey::Press);
            return Some(DPadKey::Press);
        }

        let current = self.current.to_coordinate();
        let destination = self.destination.to_coordinate();
        // If the current direction get us to a valid square and closer to the destination, stay with it

        for key in [
            self.current_direction,
            DPadKey::Right,
            DPadKey::Up,
            DPadKey::Left,
            DPadKey::Down,
        ] {
            let direction = key.to_direction();
            let new_current = (current.0 + direction.0, current.1 + direction.1);
            let new_position = NumPadKey::key_from_coordinate(new_current);
            if let Some(current_key) = new_position
                && distance(new_current, destination) < distance(current, destination)
            {
                // println!(
                //     "NumPad Moving from {:?} to {:?} in {:?}. New current - {:?}",
                //     self.current, self.destination, key, current_key
                // );
                self.current = current_key;
                self.current_direction = key;
                self.all_keys.push(self.current_direction);
                return Some(self.current_direction);
            }
        }

        unreachable!("At least one direction should work")
    }
}

struct DPad {
    remote: Box<dyn Iterator<Item = DPadKey>>,
    current: DPadKey,
    destination: DPadKey,
    current_direction: DPadKey,
    all_keys: Vec<DPadKey>,
}

impl Iterator for DPad {
    type Item = DPadKey;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.destination {
            self.destination = match self.remote.next() {
                Some(x) => x,
                None => {
                    if self.current_direction == DPadKey::Press {
                        // println!("{}", self.all_keys.iter().map(|dpadkey| dpadkey.to_char()).collect::<String>());
                        return None;
                    } else {
                        self.all_keys.push(DPadKey::Press);
                        self.current_direction = DPadKey::Press;
                        return Some(DPadKey::Press);
                    }
                }
            };
            self.all_keys.push(DPadKey::Press);
            // println!("DPad reached destination - {:?}", self.current);
            self.current_direction = DPadKey::Press;
            return Some(DPadKey::Press);
        }

        let current = self.current.to_coordinate();
        let destination = self.destination.to_coordinate();

        for key in [
            self.current_direction,
            DPadKey::Right,
            DPadKey::Up,
            DPadKey::Left,
            DPadKey::Down,
        ] {
            let direction = key.to_direction();
            let new_current = (current.0 + direction.0, current.1 + direction.1);
            let new_position = DPadKey::key_from_coordinate(new_current);
            if let Some(current_key) = new_position
                && distance(new_current, destination) < distance(current, destination)
            {
                // println!(
                //     "DPad Moving from {:?} to {:?} in {:?}. New current - {:?}",
                //     self.current, self.destination, key, current_key
                // );
                self.current = current_key;
                self.current_direction = key;
                self.all_keys.push(self.current_direction);
                return Some(self.current_direction);
            }
        }

        unreachable!("at least one dpad direction should work")
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum DPadKey {
    Up,
    Right,
    Down,
    Left,
    Press,
}

impl DPadKey {
    fn to_char(self) -> char {
        match self {
            DPadKey::Up => '^',
            DPadKey::Right => '>',
            DPadKey::Down => 'v',
            DPadKey::Left => '<',
            DPadKey::Press => 'A',
        }
    }

    fn to_direction(self) -> (i32, i32) {
        match self {
            DPadKey::Up => (-1, 0),
            DPadKey::Right => (0, 1),
            DPadKey::Down => (1, 0),
            DPadKey::Left => (0, -1),
            DPadKey::Press => (0, 0),
        }
    }

    fn to_coordinate(self) -> (i32, i32) {
        match self {
            DPadKey::Up => (0, 1),
            DPadKey::Right => (1, 2),
            DPadKey::Down => (1, 1),
            DPadKey::Left => (1, 0),
            DPadKey::Press => (0, 2),
        }
    }

    fn key_from_coordinate(coordinate: (i32, i32)) -> Option<Self> {
        match coordinate {
            (0, 1) => Some(DPadKey::Up),
            (1, 2) => Some(DPadKey::Right),
            (1, 1) => Some(DPadKey::Down),
            (1, 0) => Some(DPadKey::Left),
            (0, 2) => Some(DPadKey::Press),
            _ => None,
        }
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

    fn key_from_coordinate(coordinate: (i32, i32)) -> Option<Self> {
        match coordinate {
            (3, 1) => Some(NumPadKey::Zero),
            (2, 0) => Some(NumPadKey::One),
            (2, 1) => Some(NumPadKey::Two),
            (2, 2) => Some(NumPadKey::Three),
            (1, 0) => Some(NumPadKey::Four),
            (1, 1) => Some(NumPadKey::Five),
            (1, 2) => Some(NumPadKey::Six),
            (0, 0) => Some(NumPadKey::Seven),
            (0, 1) => Some(NumPadKey::Eight),
            (0, 2) => Some(NumPadKey::Nine),
            (3, 2) => Some(NumPadKey::Press),
            _ => None,
        }
    }
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
common::aoctest!(126384, 1234, 1234, 1234);
