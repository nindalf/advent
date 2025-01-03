use std::cmp;

use ahash::AHashMap;

type PageOrder = AHashMap<(u32, u32), cmp::Ordering>;
type Book = Vec<u32>;

#[inline]
pub fn part1(input: &str) -> u32 {
    let (books, page_order) = parse(input);

    books
        .iter()
        .filter_map(|book| {
            let (_, was_already_ordered) = is_book_ordered(book, &page_order);
            if was_already_ordered {
                return Some(book[book.len() / 2]);
            }
            None
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u32 {
    let (books, page_order) = parse(input);

    books
        .iter()
        .filter_map(|book| {
            let (sorted_book, was_already_ordered) = is_book_ordered(book, &page_order);
            if !was_already_ordered {
                return Some(sorted_book[sorted_book.len() / 2]);
            }
            None
        })
        .sum()
}

fn is_book_ordered(book: &Book, page_order: &PageOrder) -> (Vec<u32>, bool) {
    let mut sorted_book = (*book).clone();
    sorted_book.sort_unstable_by(|a, b| match page_order.get(&(*a, *b)) {
        Some(ordering) => *ordering,
        None => cmp::Ordering::Greater,
    });
    for i in 0..book.len() {
        if book[i] != sorted_book[i] {
            return (sorted_book, false);
        }
    }
    (sorted_book, true)
}

pub fn parse(input: &str) -> (Vec<Book>, PageOrder) {
    let (first, second) = input.split_once("\n\n").unwrap();

    let mut page_order = AHashMap::with_capacity(100);
    first
        .lines()
        .filter_map(|line| line.split_once("|"))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .for_each(|(lesser, greater)| {
            page_order.insert((lesser, greater), cmp::Ordering::Less);
        });

    let books = second
        .lines()
        .map(|line| line.split(',').flat_map(str::parse).collect())
        .collect();

    (books, page_order)
}

common::aoctest!(143, 6951, 123, 4121);
