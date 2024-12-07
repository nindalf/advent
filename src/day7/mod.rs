use rayon::prelude::*;

// Note on performance: The first iteration of this solution had a calculate function that accepted
// the number of possible operations (2 or 3) as a param.
// I replaced that function with specific versions calculate_2 and calculate_3 that hardcode 2 and 3.
// This improves performance dramatically because the compiler is able to generate much better code.
// %2 and /2 are 1 CPU instruction each.
// Part 1: 1.34 ms -> 269.31 µs (-79%)
// Part 2: 61.7 ms -> 53.44 ms (-13.5%)

// Note on readability/DRY: I replaced calculate_2 and calculate_3 with calculate<const N:u64>.
// It achieves the same thing because calculate<const N:u64> is monomorphized at compile-time
// to the equivalent of calculate_2 and calculate_3.

// Second note on performance: The third iteration of this was based on an algorithm that my friend
// Kushagra suggested. He wanted to know how his Kotlin solution would perform if implemented in Rust.
// I implemented it and found a 24% reduction for part 1 and _90% reduction_ in time for part 2.
// Although it looks less efficient (array operations, multiple vector operations), it ends up doing
// 85-93% fewer loop iterations.
// The time to beat is now 193µs and 5.2 ms

#[inline]
pub fn part1(input: &str) -> u64 {
    parse(input)
        .filter(|(result, operands)| calculate_k::<2>(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    parse(input)
        .filter(|(result, operands)| calculate_k::<3>(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

#[allow(dead_code)]
fn calculate<const N: u64>(result: u64, operands: &[u64]) -> bool {
    // If there are 11 operands
    // There will be 10 operations
    let operations_len = operands.len() - 1;
    // And N^10 iterations to calculate every possibile combination of operations
    let num_iterations = N.pow(operations_len as u32);

    for i in 0..num_iterations {
        let mut calculated_result = operands[0];
        let mut operations = i;
        for j in 0..operations_len {
            let operand = operands[j + 1];
            calculated_result = match operations % N {
                0 => calculated_result * operand,
                1 => calculated_result + operand,
                2 => {
                    // Formatting takes 292ms while multiplication + addition takes 61ms (80% faster)
                    // format!("{calculated_result}{operand}").parse().unwrap()
                    (calculated_result * next_power_of_10(operand)) + operand
                }
                _ => unreachable!(),
            };
            if calculated_result > result {
                break;
            }
            operations /= N;
        }
        if calculated_result == result {
            return true;
        }
    }
    false
}

fn calculate_k<const N: usize>(result: u64, operands: &[u64]) -> bool {
    let mut intermediates: Vec<u64> = vec![operands[0]];
    for operand in operands.iter().skip(1) {
        let mut temp = Vec::with_capacity(intermediates.len() * N);
        for n in intermediates {
            let addition_result = n + operand;
            if addition_result <= result {
                temp.push(addition_result);
            }
            let multiplication_result = n * operand;
            if multiplication_result <= result {
                temp.push(multiplication_result);
            }
            if N == 3 {
                let concat_result = (n * next_power_of_10(*operand)) + *operand;
                if concat_result <= result {
                    temp.push(concat_result);
                }
            }
        }
        intermediates = temp
    }

    intermediates.iter().any(|n| *n == result)
}

fn next_power_of_10(n: u64) -> u64 {
    if n == 0 {
        return 10;
    }
    let mut power = 1;
    while power <= n {
        power *= 10;
    }
    power
}

fn parse(input: &str) -> impl rayon::prelude::ParallelIterator<Item = (u64, Vec<u64>)> + use<'_> {
    input
        .par_lines()
        .filter_map(|line| line.split_once(":"))
        .map(|(result, numbers)| {
            let result = result.parse::<u64>().unwrap();
            (
                result,
                numbers
                    .split_ascii_whitespace()
                    .flat_map(str::parse)
                    .collect(),
            )
        })
}

crate::aoctest!(3749, 3245122495150, 11387, 105517128211543);
