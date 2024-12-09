#[inline]
pub fn part1(input: &str) -> usize {
    let disk = parse(input);
    let mut forward = 0;
    let mut disk_index = 0;
    let mut file_id = 0;

    let mut reverse_file = File {
        id: disk.len() / 2,
        disk_idx: disk.len() - 1,
        disk_offset: 0,
        remaining: disk[disk.len() - 1],
    };

    let total_bytes = disk.iter().step_by(2).sum();

    let mut checksum = 0;
    while disk_index < total_bytes {
        // forward points at a file
        if forward % 2 == 0 {
            for _ in 0..disk[forward] {
                checksum += disk_index * file_id;
                disk_index += 1;
                if disk_index == total_bytes {
                    break;
                }
            }
            forward += 1;
            file_id += 1;
        } else {
            // forward points to free space, let's get data from the back
            for _ in 0..disk[forward] {
                checksum += disk_index * reverse_file.id;
                disk_index += 1;
                reverse_file.remaining -= 1;
                if reverse_file.remaining == 0 {
                    reverse_file = File {
                        id: reverse_file.id - 1,
                        disk_idx: reverse_file.disk_idx - 2,
                        disk_offset: 0,
                        remaining: disk[reverse_file.disk_idx - 2],
                    }
                }
                if disk_index == total_bytes {
                    break;
                }
            }
            forward += 1;
        }
    }

    checksum
}

#[derive(Debug)]
struct File {
    id: usize,
    disk_idx: usize,
    disk_offset: usize,
    remaining: usize,
}

#[derive(Debug)]
struct FreeSpace {
    disk_offset: usize,
    internal_offset: usize,
    remaining: usize,
}

#[inline]
pub fn part2(input: &str) -> usize {
    let disk = parse(input);
    let mut files = Vec::with_capacity(disk.len() / 2 + 1);
    let mut free_spaces = Vec::with_capacity(disk.len() / 2);
    let mut disk_offset = 0;
    for i in (0..disk.len() - 1).step_by(2) {
        files.push(File {
            id: i / 2,
            disk_idx: i,
            disk_offset,
            remaining: disk[i],
        });
        disk_offset += disk[i];
        free_spaces.push(FreeSpace {
            disk_offset,
            internal_offset: 0,
            remaining: disk[i + 1],
        });
        disk_offset += disk[i + 1];
    }
    files.push(File {
        id: disk.len() / 2,
        disk_idx: disk.len() - 1,
        disk_offset,
        remaining: disk[disk.len() - 1],
    });

    let mut checksum = 0;
    for i in (0..files.len()).rev() {
        let file = &files[i];
        let free_space_idx = free_spaces
            .iter()
            .enumerate()
            .find_map(|(idx, free_space)| {
                if free_space.remaining >= file.remaining
                    && free_space.disk_offset < file.disk_offset
                {
                    return Some(idx);
                }
                None
            });

        match free_space_idx {
            // There is space, move the file
            Some(idx) => {
                let free_space = &mut free_spaces[idx];
                for _ in 0..file.remaining {
                    checksum += (free_space.disk_offset + free_space.internal_offset) * file.id;
                    free_space.internal_offset += 1;
                    free_space.remaining -= 1;
                }
            }
            // There is no space, don't move the file.
            None => {
                for j in 0..file.remaining {
                    checksum += (file.disk_offset + j) * file.id;
                }
            }
        }
    }

    checksum
}

fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| (c as usize - '0' as usize))
        .collect()
}

crate::aoctest!(1928, 6288599492129, 2858, 6321896265143);
