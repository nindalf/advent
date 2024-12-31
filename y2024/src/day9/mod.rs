use std::{cmp::Ordering, collections::BinaryHeap};

/// Performance.
/// Probably my biggest win by far - 98.7% reduction for part 2: 27.7ms -> 364.33 µs
/// Most of the time was spent scanning the free space array to find one that fit the criteria
/// Small change - instead of an array of free spaces, maintain a Vec of 10 BinaryHeaps of free spaces.
/// In each BinaryHeap the free space closest to the front is at the top.
/// Finding an appropriate free space means scanning at most 9 free spaces and finding the one that's the first from the beginning.
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

#[inline]
pub fn part2(input: &str) -> usize {
    let disk = parse(input);
    let mut files = Vec::with_capacity(disk.len() / 2 + 1);
    let mut free_spaces = vec![BinaryHeap::<FreeSpace>::with_capacity(disk.len() / 3); 10];
    let mut disk_offset = 0;
    for i in (0..disk.len() - 1).step_by(2) {
        files.push(File {
            id: i / 2,
            disk_idx: i,
            disk_offset,
            remaining: disk[i],
        });
        disk_offset += disk[i];
        let free_space = FreeSpace {
            disk_offset,
            internal_offset: 0,
            remaining: disk[i + 1],
        };
        // Push the free space into the binary heap corresponding to it's remaining space
        free_spaces[free_space.remaining].push(free_space);
        disk_offset += disk[i + 1];
    }
    files.push(File {
        id: disk.len() / 2,
        disk_idx: disk.len() - 1,
        disk_offset,
        remaining: disk[disk.len() - 1],
    });

    let mut checksum = 0;
    for file in files.iter().rev() {
        let free_space = free_spaces
            .iter()
            .skip(file.remaining) // Don't bother with free spaces smaller than necessary
            .filter_map(|heap| heap.peek()) // Get the top free space from all remaining heaps
            .filter(|free_space| free_space.disk_offset < file.disk_offset) // Only get the free spaces before the file
            .min_by(|a, b| b.cmp(a)) // Get the earliest free_space that matches the criteria
            .copied();

        match free_space {
            // There is space, move the file
            Some(mut free_space) => {
                // Pop out the free space we're about to modify
                free_spaces[free_space.remaining].pop();
                for _ in 0..file.remaining {
                    checksum += (free_space.disk_offset + free_space.internal_offset) * file.id;
                    free_space.internal_offset += 1;
                    free_space.remaining -= 1;
                }
                if free_space.remaining > 0 {
                    // Insert the newly shrunk free space into the appropriate heap
                    free_spaces[free_space.remaining].push(free_space);
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

#[derive(Debug)]
struct File {
    id: usize,
    disk_idx: usize,
    disk_offset: usize,
    remaining: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct FreeSpace {
    disk_offset: usize,
    internal_offset: usize,
    remaining: usize,
}

impl Ord for FreeSpace {
    fn cmp(&self, other: &Self) -> Ordering {
        other.disk_offset.cmp(&self.disk_offset)
    }
}

impl PartialOrd for FreeSpace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| (c as usize - '0' as usize))
        .collect()
}

common::aoctest!(1928, 6288599492129, 2858, 6321896265143);
