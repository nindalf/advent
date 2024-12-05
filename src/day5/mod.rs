use ahash::{AHashMap, AHashSet};

#[inline]
pub fn part1(input: &str) -> u32 {
    let (ordering, books) = parse(input);

    books
        .iter()
        .filter(|book| is_book_ordered(&ordering, *book))
        .map(|book| book[book.len() / 2])
        .sum()
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let (ordering, books) = parse(input);

    books
        .iter()
        .filter(|book| !is_book_ordered(&ordering, *book))
    
    0
}

fn is_book_ordered(ordering: &AHashMap<u32, AHashSet<u32>>, book:&[u32]) -> bool {
    let mut seen_so_far = AHashSet::with_capacity(book.len());
    for page in book {
        if let Some(should_appear_before) = ordering.get(&page) {
            if seen_so_far.intersection(should_appear_before).count() > 0 {
                // This page is appearing after some page it shouldn't have.
                return false;
            }
        }
        seen_so_far.insert(*page);
    }
    true
}

fn parse(input: &str) -> (AHashMap<u32, AHashSet<u32>>, Vec<Vec<u32>>) {
    let (first, second) = input.split_once("\n\n").unwrap();
    let ordering = first
        .lines()
        .filter_map(|line| line.split_once("|"))
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .fold(AHashMap::with_capacity(500), |mut result, (key, val)| {
            result
                .entry(key)
                .or_insert(AHashSet::with_capacity(50))
                .insert(val);
            result
        });

    let books = second
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|x| x.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .collect();

    (ordering, books)
}

crate::aoctest!(143, 6951, 123, 1234);
