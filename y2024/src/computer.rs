pub struct Computer<'a> {
    input: &'a str,
    pub multiply_enabled: bool,
}

#[derive(Debug)]
pub enum Instruction {
    Do,
    Dont,
    From,
    How,
    Mul(i32, i32),
    Select,
    SpecialCharacters,
    UnknownChar(char),
    What,
    When,
    Where,
    Whitespace,
    Why,
}

impl<'a> Computer<'a> {
    pub fn init(input: &'a str) -> Computer<'a> {
        Computer {
            input,
            multiply_enabled: true,
        }
    }

    pub fn next_instruction(&mut self) -> Option<Instruction> {
        if self.input.is_empty() {
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
use winnow::{ModalResult, Parser};

fn parse_do(input: &mut &str) -> ModalResult<Instruction> {
    literal("do()").map(|_| Instruction::Do).parse_next(input)
}

fn parse_dont(input: &mut &str) -> ModalResult<Instruction> {
    literal("don't()")
        .map(|_| Instruction::Dont)
        .parse_next(input)
}

fn parse_from(input: &mut &str) -> ModalResult<Instruction> {
    literal("from()")
        .map(|_| Instruction::From)
        .parse_next(input)
}

fn parse_how(input: &mut &str) -> ModalResult<Instruction> {
    literal("how()").map(|_| Instruction::How).parse_next(input)
}

fn parse_select(input: &mut &str) -> ModalResult<Instruction> {
    literal("select()")
        .map(|_| Instruction::Select)
        .parse_next(input)
}

fn parse_what(input: &mut &str) -> ModalResult<Instruction> {
    literal("what()")
        .map(|_| Instruction::What)
        .parse_next(input)
}

fn parse_when(input: &mut &str) -> ModalResult<Instruction> {
    literal("when()")
        .map(|_| Instruction::When)
        .parse_next(input)
}

fn parse_where(input: &mut &str) -> ModalResult<Instruction> {
    literal("where()")
        .map(|_| Instruction::Where)
        .parse_next(input)
}

fn parse_why(input: &mut &str) -> ModalResult<Instruction> {
    literal("why()").map(|_| Instruction::Why).parse_next(input)
}

fn parse_mul(input: &mut &str) -> ModalResult<Instruction> {
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

fn parse_gibberish(input: &mut &str) -> ModalResult<Instruction> {
    winnow::token::any
        .map(Instruction::UnknownChar)
        .parse_next(input)
}

fn parse_special_character(input: &mut &str) -> ModalResult<Instruction> {
    take_while(1.., is_special_character)
        .map(|_: &str| Instruction::SpecialCharacters)
        .parse_next(input)
}

fn parse_whitespace(input: &mut &str) -> ModalResult<Instruction> {
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

fn parse_instruction<'s>(input: &mut &'s str) -> ModalResult<(&'s str, Instruction)> {
    alt((
        parse_do,
        parse_dont,
        parse_from,
        parse_how,
        parse_what,
        parse_when,
        parse_where,
        parse_why,
        parse_mul,
        parse_select,
        parse_special_character,
        parse_whitespace,
        parse_gibberish,
    ))
    .parse_peek(input)
}
