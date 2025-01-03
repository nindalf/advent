#[inline]
pub fn part1(input: &str) -> String {
    let mut computer = parse(input);
    let output = computer.execute();
    output
        .iter()
        .map(u64::to_string)
        .collect::<Vec<String>>()
        .join(",")
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let computer = parse(input);
    let desired_output = computer.mem.clone();
    let d_len = desired_output.len();

    let mut override_a = 1;
    loop {
        // Check if this is the correct value
        let mut cloned_computer = computer.clone();
        cloned_computer.A = override_a;
        let output = cloned_computer.execute();
        if output == desired_output {
            return override_a;
        }

        // The output length is proportional to size of the override
        // Multiplying by 10 allows us to quickly converge on the real answer
        if output.len() < d_len {
            override_a *= 10;
            continue;
        }
        // If the last digit doesn't match, increment by a lot - `multiple``
        // If the penultimate digit doesn't match, increment by a bit less - `multiple/10` and so on.
        let mut multiple = 10u64.pow(override_a.ilog10() - 3);
        for (i, val) in output.iter().enumerate().rev() {
            if *val != desired_output[i] {
                override_a += multiple;
                break;
            }
            multiple /= 8;
        }
        override_a += 1;
    }
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
    fn execute(&mut self) -> Vec<u64> {
        let mut result = Vec::with_capacity(20);
        while self.is_instruction_pointer_valid() {
            if let Some(output) = self.execute_one() {
                result.push(output);
            }
        }
        result
    }

    fn execute_one(&mut self) -> Option<u64> {
        let opcode = self.mem[self.instruction_pointer];
        let operand = self.mem[self.instruction_pointer + 1];
        match opcode {
            // adv
            0 => {
                let combo_operand = self.operand_val(operand);
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
                let combo_operand = self.operand_val(operand);
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
                let combo_operand = self.operand_val(operand);
                self.instruction_pointer += 2;
                return Some(combo_operand % 8);
            }
            // bdv
            6 => {
                let combo_operand = self.operand_val(operand);
                self.B = self.A / 2u64.pow(combo_operand as u32);
                self.instruction_pointer += 2;
            }
            // cdv
            7 => {
                let combo_operand = self.operand_val(operand);
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
    "5,7,3,0".to_string(),
    "7,1,2,3,2,6,7,2,5".to_string(),
    117440,
    202356708354602
);
