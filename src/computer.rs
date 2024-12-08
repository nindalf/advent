pub struct Computer<'a> {
    input: &'a str,
    pub multiply_enabled: bool,
}

#[derive(Debug)]
pub enum Instruction {
    Do,
    Dont,
    UnknownChar(char),
    Mul(i32, i32),
    SpecialCharacters(String),
    Unknown,
    Whitespace,
}

impl<'a> Computer<'a> {
    pub fn init(input: &'a str) -> Computer<'a> {
        Computer {
            input,
            multiply_enabled: true,
        }
    }

    pub fn next_instruction(&mut self) -> Option<Instruction> {
        if self.input == "" {
            return None;
        }
        let (remaining, instruction) = parse_instruction(&mut self.input).unwrap();
        self.input = remaining;
        Some(instruction)
    }
}

use winnow::ascii::digit1;
use winnow::combinator::{alt, delimited, separated_pair};
use winnow::token::{literal, take_while};
use winnow::{PResult, Parser};

fn parse_dont<'s>(input: &mut &'s str) -> PResult<Instruction> {
    literal("don't()")
        .map(|_| Instruction::Dont)
        .parse_next(input)
}

fn parse_do<'s>(input: &mut &'s str) -> PResult<Instruction> {
    literal("do()").map(|_| Instruction::Do).parse_next(input)
}

fn parse_mul<'s>(input: &mut &'s str) -> PResult<Instruction> {
    delimited(
        literal("mul("),
        separated_pair(
            digit1.try_map(|s: &str| s.parse::<i32>()),
            literal(","),
            digit1.try_map(|s: &str| s.parse::<i32>()),
        ),
        literal(")"),
    )
    .map(|(x, y)| Instruction::Mul(x, y))
    .parse_next(input)
}

fn parse_gibberish<'s>(input: &mut &'s str) -> PResult<Instruction> {
    winnow::token::any
        .map(|c| Instruction::UnknownChar(c))
        .parse_next(input)
}

fn parse_special_character<'s>(input: &mut &'s str) -> PResult<Instruction> {
    take_while(1.., is_special_character)
        .map(|s: &str| Instruction::SpecialCharacters(s.to_string()))
        .parse_next(input)
}

fn parse_whitespace<'s>(input: &mut &'s str) -> PResult<Instruction> {
    take_while(1.., is_whitespace)
        .map(|_| Instruction::Whitespace)
        .parse_next(input)
}

fn is_special_character(c: char) -> bool {
    matches!(c, '!'..='/')
        || matches!(c, ':'..='@')
        || matches!(c, '['..='_')
        || matches!(c, '{'..='~')
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

fn parse_instruction<'s>(input: &mut &'s str) -> PResult<(&'s str, Instruction)> {
    alt((
        parse_do,
        parse_dont,
        parse_mul,
        parse_special_character,
        parse_whitespace,
        parse_gibberish,
    ))
    .parse_peek(input)
}
