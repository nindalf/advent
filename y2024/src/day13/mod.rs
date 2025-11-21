use num::integer::lcm;

/// Not on performance: Cramer's rule (229 µs, 224 µs) slightly outpeforms the Naive version (233 µs, 233 µs).
/// Parallelising this regresses performance to 286µs (+22%) and 278µs (+23%).
/// I reckon most of this time is in `scan_fmt` because that's really slow, but replacing `scan_fmt` with `regex::Captures`
/// regresses performance to 256µs and 257µs.
/// Implementing a handwritten parser with winnow blows both out of the water, reducing time to 34.35µs (-84%) and 34.3µs (-84%)
/// Benchmarking the parser alone
/// scan_fmt = 227.97 µs
/// winnow = 34.805 µs (-87%)
/// The actual processing for this problem is about 1-3µs.
/// Note that benching the parse() function only works if it returns a Vec<_>, not an impl Iterator, which returns within a few ns.
#[inline]
pub fn part1(input: &str) -> i64 {
    parse(input)
        .filter_map(|(eq1, eq2)| solve_cramer_rule(eq1, eq2))
        .sum()
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
pub struct Equation {
    op1: i64,
    op2: i64,
    result: i64,
}

#[inline]
pub fn parse(input: &str) -> impl Iterator<Item = (Equation, Equation)> + use<'_> {
    input
        .split("\n\n")
        .flat_map(|mut part| parse_machine(&mut part))
}

#[allow(dead_code)]
fn parse_with_scan_fmt(input: &str) -> anyhow::Result<(Equation, Equation)> {
    let (x_op1, y_op1, x_op2, y_op2, x_result, y_result) = scan_fmt::scan_fmt!(
        input,
        "Button A: X+{d}, Y+{d}\nButton B: X+{d}, Y+{d}\nPrize: X={d}, Y={d}",
        i64,
        i64,
        i64,
        i64,
        i64,
        i64
    )?;
    Ok((
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
    ))
}

use winnow::ascii::digit1;
use winnow::combinator::{alt, delimited, preceded, separated_pair};
use winnow::token::literal;
use winnow::{ModalResult, Parser};

fn parse_machine(input: &mut &str) -> ModalResult<(Equation, Equation)> {
    let (x_op1, y_op1) = parse_line(input)?;
    let (x_op2, y_op2) = parse_line(input)?;
    let (x_result, y_result) = parse_prize(input)?;
    Ok((
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
    ))
}

fn parse_line(input: &mut &str) -> ModalResult<(i64, i64)> {
    delimited(
        alt((literal("Button A: X+"), literal("Button B: X+"))),
        separated_pair(
            digit1.try_map(|s: &str| s.parse::<i64>()),
            literal(", Y+"),
            digit1.try_map(|s: &str| s.parse::<i64>()),
        ),
        literal("\n"),
    )
    .parse_next(input)
}

fn parse_prize(input: &mut &str) -> ModalResult<(i64, i64)> {
    preceded(
        literal("Prize: X="),
        separated_pair(
            digit1.try_map(|s: &str| s.parse::<i64>()),
            literal(", Y="),
            digit1.try_map(|s: &str| s.parse::<i64>()),
        ),
    )
    .parse_next(input)
}

common::aoctest!(480, 25751, 875318608908, 108528956728655);
