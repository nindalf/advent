use std::cmp;

use ahash::AHashMap;

type PageOrder = AHashMap<(u32, u32), cmp::Ordering>;
type Book = Vec<u32>;

#[inline]
pub fn part1(input: &str) -> u32 {
    let (mut books, page_order) = parse(input);

    books
        .iter_mut()
        .filter_map(|book| {
            let was_already_ordered = sort_book(book, &page_order);
            if was_already_ordered {
                return Some(book[book.len() / 2]);
            }
            None
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u32 {
    let (mut books, page_order) = parse(input);

    books
        .iter_mut()
        .filter_map(|book| {
            let was_already_ordered = sort_book(book, &page_order);
            if !was_already_ordered {
                return Some(book[book.len() / 2]);
            }
            None
        })
        .sum()
}

fn sort_book<'a>(book: &'a mut Book, page_order: &PageOrder) -> bool {
    let hash_before = generate_hash(&book);
    book.sort_unstable_by(|a, b| match page_order.get(&(*a, *b)) {
        Some(ordering) => *ordering,
        None => cmp::Ordering::Greater,
    });
    let hash_after = generate_hash(&book);
    hash_before == hash_after
}

use std::hash::{Hash, Hasher};

fn generate_hash(arr: &[u32]) -> u64 {
    // Create a new hasher
    let mut hasher = ahash::AHasher::default();

    // Hash each element of the array along with its index to preserve order
    for (index, &num) in arr.iter().enumerate() {
        (index, num).hash(&mut hasher);
    }

    // Return the hash value
    hasher.finish()
}

fn parse(input: &str) -> (Vec<Book>, PageOrder) {
    let (first, second) = input.split_once("\n\n").unwrap();

    let mut page_order = AHashMap::with_capacity(100);
    first
        .lines()
        .filter_map(|line| line.split_once("|"))
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .for_each(|(lesser, greater)| {
            page_order.insert((lesser, greater), cmp::Ordering::Less);
        });

    let books = second
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|x| x.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .collect();

    (books, page_order)
}

crate::aoctest!(143, 6951, 123, 4121);
