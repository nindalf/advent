use num::integer::lcm;
use scan_fmt::scan_fmt;

/// Not on performance: Cramer's rule (229 µs, 224 µs) slightly outpeforms the Naive version (233 µs, 233 µs).
/// Parallelising this regresses performance to 286µs (+22%) and 278µs (+23%).
/// I reckon most of this time is in `scan_fmt` because that's really slow, but replacing `scan_fmt` with `regex::Captures`
/// regresses performance to 256µs and 257µs.
#[inline]
pub fn part1(input: &str) -> i64 {
    parse(input).filter_map(|(eq1, eq2)| solve_cramer_rule(eq1, eq2)).sum()
}

#[inline]
pub fn part2(input: &str) -> i64 {
    parse(input)
        .map(|(eq1, eq2)| {
            (
                Equation {
                    op1: eq1.op1,
                    op2: eq1.op2,
                    result: eq1.result + 10000000000000,
                },
                Equation {
                    op1: eq2.op1,
                    op2: eq2.op2,
                    result: eq2.result + 10000000000000,
                },
            )
        })
        .filter_map(|(eq1, eq2)| solve_cramer_rule(eq1, eq2))
        .sum()
}

/// Note on correctness: the input with 10000000009164 and 10000000002799 gave the wrong answer
/// on the first version of this function. It incorrectly assumed that if dividend % divisor == 0
/// then a solution exists.
#[allow(dead_code)]
fn solve_naive(one: Equation, two: Equation) -> Option<i64> {
    let lcm = lcm(one.op1, two.op1);
    let mul_one = lcm / one.op1;
    let mul_two = lcm / two.op1;
    let dividend = one.result * mul_one - two.result * mul_two;
    let divisor = one.op2 * mul_one - two.op2 * mul_two;
    if divisor == 0 {
        return None;
    }
    let b = dividend / divisor;
    let a = (one.result - one.op2 * b) / one.op1;
    if (one.op1 * a + one.op2 * b, two.op1 * a + two.op2 * b) == (one.result, two.result) {
        Some(a * 3 + b)
    } else {
        None
    }
}

/// Thanks to https://old.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
/// for explaining.
fn solve_cramer_rule(one: Equation, two: Equation) -> Option<i64> {
    let det = one.op1 * two.op2 - one.op2 * two.op1;
    let a = (one.result * two.op2 - two.result * one.op2) / det;
    let b = (one.op1 * two.result - two.op1 * one.result) / det;
    if (one.op1 * a + one.op2 * b, two.op1 * a + two.op2 * b) == (one.result, two.result) {
        Some(a * 3 + b)
    } else {
        None
    }
}

#[derive(Debug, Copy, Clone)]
struct Equation {
    op1: i64,
    op2: i64,
    result: i64,
}

fn parse(input: &str) -> impl Iterator<Item = (Equation, Equation)> + use<'_> {
    input
        .split("\n\n")
        .flat_map(|part| {
            scan_fmt!(
                part,
                "Button A: X+{d}, Y+{d}\nButton B: X+{d}, Y+{d}\nPrize: X={d}, Y={d}",
                i64,
                i64,
                i64,
                i64,
                i64,
                i64
            )
        })
        .map(|(x_op1, y_op1, x_op2, y_op2, x_result, y_result)| {
            (
                Equation {
                    op1: x_op1,
                    op2: x_op2,
                    result: x_result,
                },
                Equation {
                    op1: y_op1,
                    op2: y_op2,
                    result: y_result,
                },
            )
        })
}
common::aoctest!(480, 25751, 875318608908, 108528956728655);
