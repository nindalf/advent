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

#[inline]
pub fn part1(input: &str) -> u64 {
    parse(input)
        .filter(|(result, operands)| calculate::<2>(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    parse(input)
        .filter(|(result, operands)| calculate::<3>(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

fn calculate<const N: u64>(result: u64, operands: &[u64]) -> bool {
    // If there are 11 operands
    let operations_len = operands.len() - 1; // There will be 10 operations
    let num_iterations = N.pow(operations_len as u32); // And N^10 iterations to calculate every possibile combination of operations
    for i in 0 .. num_iterations {
        let mut calculated_result = operands[0];
        let mut operations = i;
        for j in 0 .. operations_len {
            calculated_result = match operations % N {
                0 => {
                    calculated_result + operands[j+1]
                },
                1 => {
                    calculated_result * operands[j+1]
                },
                2 => {
                    // Formatting takes 292ms while multiplication + addition takes 61ms (80% faster)
                    //format!("{}{}", calculated_result, operands[j+1]).parse().unwrap()
                    (calculated_result * next_power_of_10(operands[j+1])) + operands[j+1]
                },
                _ => unreachable!(),
            };
            if calculated_result > result {
                break;
            }
            operations = operations/N;
        }
        if calculated_result == result {
            return true;
        }
    }
    false
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

fn parse(input: &str) ->impl rayon::prelude::ParallelIterator<Item = (u64, Vec<u64>)> + use<'_> {
    input.par_lines()
        .filter_map(|line| line.split_once(":"))
        .map(|(result, numbers)| {
            let result = result.parse::<u64>().unwrap();
            (result, numbers.split_ascii_whitespace()
                .flat_map(str::parse)
                .collect())
        })
}

crate::aoctest!(3749, 3245122495150, 11387, 105517128211543);