use std::hash::Hash;

use ahash::{AHashMap, AHashSet};

#[inline]
pub fn part1(input: &str) -> u64 {
    let (mut values, instructions) = parse(input);
    let mut result = 0;
    for (output, _) in instructions.iter() {
        let output_val = evaluate(output, &mut values, &instructions);
        if let Some(index) = output.strip_prefix("z") {
            let idx: usize = index.parse().unwrap();
            result |= (output_val as u64) << idx
        }
    }
    result
}

#[inline]
pub fn part2(input: &str) -> String {
    let (_, instructions) = parse(input);
    let reversed_instructions: AHashMap<Instruction, &str> = instructions.iter().fold(
        AHashMap::with_capacity(instructions.len()),
        |mut acc, (key, val)| {
            acc.insert(*val, key);
            // Reverse the order of operands and store that too.
            // These operations are commutative
            use Instruction::*;
            match val {
                And(l, r) => acc.insert(And(r, l), key),
                Or(l, r) => acc.insert(Or(r, l), key),
                Xor(l, r) => acc.insert(Xor(r, l), key),
            };
            acc
        },
    );

    let (mut x_keys, mut y_keys, mut z_keys) = (
        Vec::with_capacity(50),
        Vec::with_capacity(50),
        Vec::with_capacity(50),
    );
    for i in 0..46 {
        x_keys.push(format!("x{:02}", i));
        y_keys.push(format!("y{:02}", i));
        z_keys.push(format!("z{:02}", i));
    }

    let (mut previous_carry_name, _) = find_instruction_name(
        &reversed_instructions,
        &Instruction::And(&x_keys[0], &y_keys[0]),
    );
    let mut mismatches = AHashSet::with_capacity(8);
    // hmk, x16
    for i in 1..45 {
        let instruction = Instruction::Xor(&x_keys[i], &y_keys[i]);
        let (xor_i_name, mismatch) = find_instruction_name(&reversed_instructions, &instruction);
        if mismatch {
            let (one, two) = find_mismatches(&instruction, &instructions[xor_i_name]);
            mismatches.insert(one.to_string());
            mismatches.insert(two.to_string());
        }

        let instruction = Instruction::And(&x_keys[i], &y_keys[i]);
        let (and_i_name, mismatch) = find_instruction_name(&reversed_instructions, &instruction);
        if mismatch {
            let (one, two) = find_mismatches(&instruction, &instructions[and_i_name]);
            mismatches.insert(one.to_string());
            mismatches.insert(two.to_string());
        }

        let instruction = Instruction::And(xor_i_name, previous_carry_name);
        let (intermediate_name, mismatch) =
            find_instruction_name(&reversed_instructions, &instruction);
        if mismatch {
            let (one, two) = find_mismatches(&instruction, &instructions[intermediate_name]);
            mismatches.insert(one.to_string());
            mismatches.insert(two.to_string());
        }

        let instruction = Instruction::Or(intermediate_name, and_i_name);
        let (new_carry_name, mismatch) =
            find_instruction_name(&reversed_instructions, &instruction);
        if mismatch {
            let (one, two) = find_mismatches(&instruction, &instructions[new_carry_name]);
            mismatches.insert(one.to_string());
            mismatches.insert(two.to_string());
        }

        let instruction = Instruction::Xor(xor_i_name, previous_carry_name);
        let (result_i_name, mismatch) = find_instruction_name(&reversed_instructions, &instruction);
        if mismatch {
            let (one, two) = find_mismatches(&instruction, &instructions[result_i_name]);
            mismatches.insert(one.to_string());
            mismatches.insert(two.to_string());
        }

        previous_carry_name = new_carry_name;
    }
    let mut mismatches_vec: Vec<_> = mismatches.into_iter().collect();
    mismatches_vec.sort();
    mismatches_vec.join(",")
}

// Find the actual instruction
// Or find an instruction with the same operation and matching one operand
fn find_instruction_name<'a>(
    reversed_instructions: &AHashMap<Instruction, &'a str>,
    instruction: &Instruction,
) -> (&'a str, bool) {
    if let Some(name) = reversed_instructions.get(instruction) {
        return (name, false);
    }

    let (_, backup) = reversed_instructions
        .iter()
        .find(|(i, _)| match (i, instruction) {
            (Instruction::And(l0, r0), Instruction::And(l1, r1)) => {
                (l0 == l1) || (r0 == r1) || (l0 == r1) || (l1 == r0)
            }
            (Instruction::Or(l0, r0), Instruction::Or(l1, r1)) => {
                (l0 == l1) || (r0 == r1) || (l0 == r1) || (l1 == r0)
            }
            (Instruction::Xor(l0, r0), Instruction::Xor(l1, r1)) => {
                (l0 == l1) || (r0 == r1) || (l0 == r1) || (l1 == r0)
            }
            _ => false,
        })
        .expect("at least one instruction should match");

    (backup, true)
}

fn find_mismatches(og: &Instruction, found: &Instruction) -> (String, String) {
    match (og, found) {
        (Instruction::And(l0, r0), Instruction::And(l1, r1)) => {
            find_mismatched_strings(l0, r0, l1, r1)
        }
        (Instruction::Or(l0, r0), Instruction::Or(l1, r1)) => {
            find_mismatched_strings(l0, r0, l1, r1)
        }
        (Instruction::Xor(l0, r0), Instruction::Xor(l1, r1)) => {
            find_mismatched_strings(l0, r0, l1, r1)
        }
        _ => ("".to_string(), "".to_string()),
    }
}

fn find_mismatched_strings(l0: &str, r0: &str, l1: &str, r1: &str) -> (String, String) {
    match ((l0 == l1), (r0 == r1), (l0 == r1), (l1 == r0)) {
        (true, _, _, _) => (r0.to_string(), r1.to_string()),
        (_, true, _, _) => (l0.to_string(), l1.to_string()),
        (_, _, true, _) => (l1.to_string(), r0.to_string()),
        (_, _, _, true) => (l0.to_string(), r1.to_string()),
        _ => ("".to_string(), "".to_string()),
    }
}

#[allow(dead_code)]
fn failed_part2(input: &str) {
    let (_, instructions) = parse(input);
    let mut values: AHashMap<&str, u8> = AHashMap::with_capacity(500);
    let (mut x_keys, mut y_keys, mut z_keys) = (
        Vec::with_capacity(50),
        Vec::with_capacity(50),
        Vec::with_capacity(50),
    );
    for i in 0..46 {
        x_keys.push(format!("x{:02}", i));
        y_keys.push(format!("y{:02}", i));
        z_keys.push(format!("z{:02}", i));
    }
    // let mut keys = vec![];
    for i in 1..45 {
        values.insert(&x_keys[i - 1], 1);
        values.insert(&y_keys[i - 1], 1);

        for (x, y, expected) in [(0, 0, 1), (0, 1, 0), (1, 0, 0), (1, 1, 1)] {
            values.insert(&x_keys[i], x);
            values.insert(&y_keys[i], y);
            let output_val = evaluate(&z_keys[i], &mut values, &instructions);
            if output_val != expected {}
        }

        values.insert(&x_keys[i - 1], 0);
        values.insert(&y_keys[i - 1], 0);
    }
}

fn evaluate<'a>(
    output: &'a str,
    calculated_values: &mut AHashMap<&'a str, u8>,
    instructions: &'a AHashMap<&'a str, Instruction>,
) -> u8 {
    if (output.starts_with("x") || output.starts_with("y"))
        && calculated_values.contains_key(&output)
    {
        return calculated_values[&output];
    }

    let output_val = match instructions[&output] {
        Instruction::And(op1, op2) => {
            evaluate(op1, calculated_values, instructions)
                & evaluate(op2, calculated_values, instructions)
        }
        Instruction::Or(op1, op2) => {
            evaluate(op1, calculated_values, instructions)
                | evaluate(op2, calculated_values, instructions)
        }
        Instruction::Xor(op1, op2) => {
            evaluate(op1, calculated_values, instructions)
                ^ evaluate(op2, calculated_values, instructions)
        }
    };

    calculated_values.insert(output, output_val);
    output_val
}

fn parse(input: &str) -> (AHashMap<&str, u8>, AHashMap<&str, Instruction>) {
    let (wires, gates) = input.split_once("\n\n").expect("input is well formed");

    let initial_values = wires
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(wire, val)| (wire, val.parse::<u8>().unwrap()))
        .fold(AHashMap::with_capacity(100), |mut acc, pair| {
            acc.insert(pair.0, pair.1);
            acc
        });

    let instructions = gates
        .lines()
        .filter_map(|line| {
            let mut i = line.split_ascii_whitespace();
            let op1 = i.next()?;
            let operation = i.next()?;
            let op2 = i.next()?;
            let _ = i.next()?; // "->"
            let result = i.next()?;
            use Instruction::*;
            let instruction = match operation {
                "AND" => And(op1, op2),
                "OR" => Or(op1, op2),
                "XOR" => Xor(op1, op2),
                _ => unreachable!("Invalid instruction"),
            };
            Some((result, instruction))
        })
        .fold(AHashMap::with_capacity(250), |mut acc, pair| {
            acc.insert(pair.0, pair.1);
            acc
        });

    (initial_values, instructions)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

common::aoctest!(
    2024,
    66055249060558,
    "".to_owned(),
    "fcd,fhp,hmk,rvf,tpc,z16,z20,z33".to_owned()
);
