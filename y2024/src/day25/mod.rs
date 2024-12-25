#[inline]
pub fn part1(input: &str) -> usize {
    let (locks, keys) = parse(input);
    (0..locks.len())
        .flat_map(|i| (0..keys.len()).map(move |j| (i, j)))
        .filter(|(i, j)| does_lock_fit_key(locks[*i], keys[*j]))
        .count()
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    0
}

fn does_lock_fit_key(lock: [u8; 5], key: [u8; 5]) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] >= 6 {
            return false;
        }
    }
    true
}

fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let locks: Vec<_> = input
        .split("\n\n")
        .map(|lock| {
            lock.lines()
                .enumerate()
                .map(|(idx, line)| {
                    line.chars()
                        .map(move |c| if c == '#' { idx as u8 } else { 0 })
                })
                .fold([0u8; 5], |mut acc, mut line| {
                    for x in acc.iter_mut() {
                        *x = *x.max(&mut line.next().unwrap());
                    }
                    acc
                })
        })
        .filter(|arr| arr[0] != 6)
        .collect();

    let keys: Vec<_> = input
        .split("\n\n")
        .map(|key| {
            key.lines()
                .rev() // Only difference
                .enumerate()
                .map(|(idx, line)| {
                    line.chars()
                        .map(move |c| if c == '#' { idx as u8 } else { 0 })
                })
                .fold([0u8; 5], |mut acc, mut line| {
                    for x in acc.iter_mut() {
                        *x = *x.max(&mut line.next().unwrap());
                    }
                    acc
                })
        })
        .filter(|arr| arr[0] != 6)
        .collect();
    (locks, keys)
}

common::aoctest!(3, 3397, 1234, 1234);
