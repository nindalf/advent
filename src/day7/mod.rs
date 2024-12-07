use rayon::prelude::*;

// Note on performance: calculate2 and calculate3 only differ in a couple of lines and could 
// accept 2/3 as a paramater and still work. The first iteration of this solution did that.
// However, performance improves dramatically by hardcoding 2 and 3 as constants.
// The compiler is able to generate much better code. %2 and /2 are 1 CPU instruction each.
// Part 1: 1.34 ms -> 269.31 µs (-79%)
// Part 2: 61.7 ms -> 53.44 ms (-13.5%)

#[inline]
pub fn part1(input: &str) -> u64 {
    parse(input)
        .filter(|(result, operands)| calculate_2(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u64 {
    parse(input)
        .filter(|(result, operands)| calculate_3(*result, operands))
        .map(|(result, _)| result)
        .sum()
}


fn calculate_2(result: u64, operands: &[u64]) -> bool {
    // If there are 11 operands
    let operations_len = operands.len() - 1; // There will be 10 operations
    let num_iterations = 2u64.pow(operations_len as u32); // And 2^10 iterations to calculate every possibile combination of operations
    for i in 0 .. num_iterations {
        let mut calculated_result = operands[0];
        let mut operations = i;
        for j in 0 .. operations_len {
            calculated_result = match operations % 2 {
                0 => {
                    calculated_result + operands[j+1]
                },
                1 => {
                    calculated_result * operands[j+1]
                },
                _ => unreachable!(),
            };
            if calculated_result > result {
                break;
            }
            operations = operations/2;
        }
        if calculated_result == result {
            return true;
        }
        
    }
    false
}

fn calculate_3(result: u64, operands: &[u64]) -> bool {
    // If there are 11 operands
    let operations_len = operands.len() - 1; // There will be 10 operations
    let num_iterations = 3u64.pow(operations_len as u32); // And 3^10 iterations to calculate every possibile combination of operations
    for i in 0 .. num_iterations {
        let mut calculated_result = operands[0];
        let mut operations = i;
        for j in 0 .. operations_len {
            calculated_result = match operations % 3 {
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
                }
                _ => unreachable!(),
            };
            if calculated_result > result {
                break;
            }
            operations = operations/3;
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