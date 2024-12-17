use rayon::prelude::*;

#[inline]
pub fn part1(input: &str) -> String {
    let mut computer = parse(input);
    let mut result = Vec::with_capacity(20);
    while computer.is_instruction_pointer_valid() {
        if let Some(output) = computer.execute_one() {
            result.push(output.to_string());
        }
    }
    result.join(",")
}

#[inline]
pub fn part2(input: &str) -> u32 {
    let computer = parse(input);

    (0..10_000)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|override_a| {
            let mut clone_computer = computer.clone();
            clone_computer.A = *override_a as u64;
            clone_computer.execute_with_desired_output(&computer.mem)
        })
        .unwrap()
}

#[allow(non_snake_case)]
#[derive(Clone)]
struct Computer {
    A: u64,
    B: u64,
    C: u64,
    mem: Vec<u64>,
    instruction_pointer: usize,
}

impl Computer {
    fn execute_with_desired_output(&mut self, desired_output: &[u64]) -> bool {
        let mut idx = 0;
        while self.is_instruction_pointer_valid() {
            if let Some(output) = self.execute_one() {
                if output != desired_output[idx] {
                    return false;
                }
                idx += 1;
            }
        }
        idx == desired_output.len()
    }

    fn execute_one(&mut self) -> Option<u64> {
        let opcode = self.mem[self.instruction_pointer];
        let operand = self.mem[self.instruction_pointer + 1];
        let combo_operand = self.operand_val(operand);
        match opcode {
            // adv
            0 => {
                self.A /= 2u64.pow(combo_operand as u32);
                self.instruction_pointer += 2;
            }
            // bxl
            1 => {
                self.B ^= operand;
                self.instruction_pointer += 2;
            }
            // bst
            2 => {
                self.B = combo_operand % 8;
                self.instruction_pointer += 2;
            }
            // jnz
            3 => {
                if self.A != 0 {
                    self.instruction_pointer = operand as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            // bxc
            4 => {
                self.B ^= self.C;
                self.instruction_pointer += 2;
            }
            // out
            5 => {
                self.instruction_pointer += 2;
                return Some(combo_operand % 8);
            }
            // bdv
            6 => {
                self.B = self.A / 2u64.pow(combo_operand as u32);
                self.instruction_pointer += 2;
            }
            // cdv
            7 => {
                self.C = self.A / 2u64.pow(combo_operand as u32);
                self.instruction_pointer += 2;
            }
            _ => unreachable!("Invalid opcode"),
        };
        None
    }

    fn is_instruction_pointer_valid(&self) -> bool {
        self.instruction_pointer < self.mem.len()
    }

    fn operand_val(&self, operand: u64) -> u64 {
        match operand {
            n @ 0..=3 => n,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => unreachable!("Invalid operand"),
        }
    }
}

fn parse(input: &str) -> Computer {
    let (registers, memory) = input.split_once("\n\n").expect("input is valid");
    #[allow(non_snake_case)]
    let (A, B, C) = scan_fmt::scan_fmt!(
        registers,
        "Register A: {d}\nRegister B: {d}\nRegister C: {d}",
        u64,
        u64,
        u64
    )
    .unwrap();

    let mem = memory
        .strip_prefix("Program: ")
        .expect("memory starts with Program:")
        .chars()
        .filter_map(|c| {
            if !c.is_ascii_digit() {
                return None;
            }
            Some((c as u8 - b'0') as u64)
        })
        .collect();

    let instruction_pointer = 0;

    Computer {
        A,
        B,
        C,
        mem,
        instruction_pointer,
    }
}

common::aoctest!(
    "4,6,3,5,6,3,5,2,1,0".to_string(),
    "7,1,2,3,2,6,7,2,5".to_string(),
    117440,
    1234
);
