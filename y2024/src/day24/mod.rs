use ahash::AHashMap;

#[inline]
pub fn part1(input: &str) -> u64 {
    let (mut values, instructions) = parse(input);
    let mut intermediate = [0; 64];
    for (output, _) in instructions.iter() {
        let output_val = evaluate(output, &mut values, &instructions);
        if let Some(index) = output.strip_prefix("z") {
            let idx: usize = index.parse().unwrap();
            intermediate[idx] = output_val;
        }
    }
    let mut result = 0;
    for (idx, bit) in intermediate.iter().enumerate() {
        result |= (*bit as u64) << idx
    }
    result
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    0
}

fn evaluate<'a>(
    output: &'a str,
    calculated_values: &mut AHashMap<&'a str, u8>,
    instructions: &'a AHashMap<&str, Instruction>,
) -> u8 {
    if calculated_values.contains_key(output) {
        // println!("Returning calculated value for {output} - {}", calculated_values[output]);
        return calculated_values[output];
    }
    // println!("Evaluating {output}");
    let output_val = match instructions[output] {
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
    // println!("Evaluated {output} successfully - {output_val}");
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

enum Instruction<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

common::aoctest!(2024, 66055249060558, 1234, 1234);
