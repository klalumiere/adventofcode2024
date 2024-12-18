use std::{fs, thread::{self, JoinHandle}};

use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc(usize),
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instruction_pointer: usize,
}

impl Instruction {
    fn from(opcode: usize, operand: usize) -> Instruction {
        match opcode {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc(operand),
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!("Unknown opcode: {opcode}"),
        }
    }

    fn convert_to_opcode(&self) -> (usize, usize) {
        match self {
            Instruction::Adv(operand) => (0usize, *operand),
            Instruction::Bxl(operand) => (1usize, *operand),
            Instruction::Bst(operand) => (2usize, *operand),
            Instruction::Jnz(operand) => (3usize, *operand),
            Instruction::Bxc(operand) => (4usize, *operand),
            Instruction::Out(operand) => (5usize, *operand),
            Instruction::Bdv(operand) => (6usize, *operand),
            Instruction::Cdv(operand) => (7usize, *operand),
        }
    }

    fn run(&self, computer: & mut Computer, stdout: & mut Vec<usize>) {
        match self {
            Instruction::Adv(operand) => self.run_adv(*operand, computer, stdout),
            Instruction::Bxl(operand) => self.run_bxl(*operand, computer, stdout),
            Instruction::Bst(operand) => self.run_bst(*operand, computer, stdout),
            Instruction::Jnz(operand) => self.run_jnz(*operand, computer, stdout),
            Instruction::Bxc(operand) => self.run_bxc(*operand, computer, stdout),
            Instruction::Out(operand) => self.run_out(*operand, computer, stdout),
            Instruction::Bdv(operand) => self.run_bdv(*operand, computer, stdout),
            Instruction::Cdv(operand) => self.run_cdv(*operand, computer, stdout),
        }
    }

    // - The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
    fn run_adv(&self, operand: usize, computer: & mut Computer, _: & mut Vec<usize>) {
        let denominator = 2usize.pow(computer.get_combo_operand(operand) as u32);
        computer.register_a /= denominator;
        computer.instruction_pointer += 2
    }
    // - The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
    fn run_bxl(&self, operand: usize, computer: & mut Computer, _: & mut Vec<usize>) {
        computer.register_b ^= operand;
        computer.instruction_pointer += 2
    }
    // - The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    fn run_bst(&self, operand: usize, computer: & mut Computer, _: & mut Vec<usize>) {
        computer.register_b = computer.get_combo_operand(operand) % 8;
        computer.instruction_pointer += 2
    }
    // - The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    fn run_jnz(&self, operand: usize, computer: & mut Computer, _: & mut Vec<usize>) {
        if computer.register_a == 0 {
            computer.instruction_pointer += 2;
        } else {
            computer.instruction_pointer = operand;
        }
        // TODO: computer.instruction_pointer = ?
    }
    // - The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    fn run_bxc(&self, _: usize, computer: & mut Computer, _: & mut Vec<usize>) {
        computer.register_b ^= computer.register_c;
        computer.instruction_pointer += 2
    }
    // - The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    fn run_out(&self, operand: usize, computer: & mut Computer, stdout: & mut Vec<usize>) {
        stdout.push(computer.get_combo_operand(operand) % 8);
        computer.instruction_pointer += 2
    }
    // - The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
    fn run_bdv(&self, operand: usize, computer: & mut Computer, _: & mut Vec<usize>) {
        let denominator = 2usize.pow(computer.get_combo_operand(operand) as u32);
        computer.register_b = computer.register_a / denominator;
        computer.instruction_pointer += 2
    }
    // - The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
    fn run_cdv(&self, operand: usize, computer: & mut Computer, _: & mut Vec<usize>) {
        let denominator = 2usize.pow(computer.get_combo_operand(operand) as u32);
        computer.register_c = computer.register_a / denominator;
        computer.instruction_pointer += 2
    }
}

impl Program {
    fn from(line: &str) -> Program {
        let mut instructions: Vec<Instruction> = Vec::new();
        let program_regex = Regex::new(r"Program: (\d(,\d)*)").unwrap();
        let program_str = program_regex.captures(line).expect("a program").get(1).expect("a match").as_str();
        let mut integers = program_str.split(",");
        while let Some(opcode_as_str) = integers.next() {
            let opcode = opcode_as_str.parse().expect("an integer");
            let operand = integers.next().expect("an integer as string").parse().expect("an integer");
            instructions.push(Instruction::from(opcode, operand));
        }
        Program { instructions }
    }

    fn convert_to_opcodes(&self) -> Vec<usize> {
        let mut opcodes: Vec<usize> = Vec::new();
        for instruction in &self.instructions {
            let (opcode, operand) = instruction.convert_to_opcode();
            opcodes.push(opcode);
            opcodes.push(operand);
        }
        opcodes
    }
}

impl Computer {
    fn from(content: &str) -> Computer {
        let register_a = Regex::new(r"Register A: (\d+)").unwrap();
        let register_b = Regex::new(r"Register B: (\d+)").unwrap();
        let register_c = Regex::new(r"Register C: (\d+)").unwrap();
        let mut lines = content.lines();
        
        let line = lines.next().expect("a line");
        let register_a = register_a.captures(line).unwrap()[1].parse::<usize>().expect("a number");
        let line = lines.next().expect("a line");
        let register_b = register_b.captures(line).unwrap()[1].parse::<usize>().expect("a number");
        let line = lines.next().expect("a line");
        let register_c = register_c.captures(line).unwrap()[1].parse::<usize>().expect("a number");

        Computer {  register_a, register_b, register_c, instruction_pointer: 0 }
    }

    fn get_combo_operand(&self, operand_index: usize) -> usize {
        match operand_index {
            0..=3 => operand_index,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand_index: {operand_index}"),
        }
    }

    fn run_instruction(& mut self, instruction: Instruction, stdout: &mut Vec<usize>) {
        instruction.run(self, stdout)
    }

    #[allow(dead_code)]
    fn run(& mut self, program: &Program) -> Vec<usize> {
        let mut stdout: Vec<usize> = Vec::new();
        if program.instructions.is_empty(){
            return stdout;
        }
        let last_valid_index_mod2 = 2*(program.instructions.len() - 1);
        while self.instruction_pointer <= last_valid_index_mod2 {
            assert!(self.instruction_pointer % 2 == 0);
            let instruction = program.instructions[self.instruction_pointer / 2usize];
            self.run_instruction(instruction, &mut stdout);
        }
        stdout
    }

    fn copy_program_once(& mut self, program: &Program) -> Option<usize> {
        let initial_register_a = self.register_a;
        let target_opcodes = program.convert_to_opcodes();
        let mut stdout: Vec<usize> = Vec::new();
        let last_valid_index_mod2 = 2*(program.instructions.len() - 1);
        while self.instruction_pointer <= last_valid_index_mod2 {
            assert!(self.instruction_pointer % 2 == 0);
            let instruction = program.instructions[self.instruction_pointer / 2usize];
            self.run_instruction(instruction, &mut stdout);
            for (target, actual) in stdout.iter().zip(&target_opcodes) {
                if target != actual {
                    return None;
                }
            }
            if stdout.len() > target_opcodes.len() {
                return None
            }
        }
        if stdout.len() == target_opcodes.len() {
            return Some(initial_register_a);
        }
        None
    }

    fn copy_program(&self, program: &Program) -> Option<isize> {
        let mut handles: Vec<JoinHandle<Option<isize>>> = vec![];
        for i in 1..10 {
            let computer = *self;
            let program = program.clone();
            let handle = thread::spawn(move || {
                for j in (i*10000000000usize)..((i+1)*10000000000usize) {
                    if j % 10000000 == 0 {
                        println!("Trying with register_a={j}");
                    }
                    let mut new_computer = Computer { register_a: j, ..computer };
                    if let Some(register_a) = new_computer.copy_program_once(&program) {
                        println!("YOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO I FOOOOOOOOOOUND A SOLUTIONNNNNNNNNNNNNNNNNNNNNNNNN {register_a}");
                        return Some(register_a as isize);
                    }
                }
                None
            });
            handles.push(handle);
        }
        let mut result: Option<isize> = None;
        for handle in handles {
            if let Some(register_a) = handle.join().unwrap() {
                result = Some(register_a);
                println!("Found a solution {register_a}");
            }
        }
        result
    }
}

fn parse_program(content: &str) -> Program {
    let mut program: Option<Program> = None;
    for line in content.lines() {
        if line.contains("Program") {
            program = Some(Program::from(line));
        }
    }
    program.expect("a program")
}

fn run_decompiled_program() -> Vec<usize> {
    let mut stdout: Vec<usize> = Vec::new();
    let mut register_a = 45483412usize;
    let mut register_b = 0usize;
    let mut register_c = 0usize;
    loop {
        register_b = register_a % 8;
        register_b ^= 3;
        register_c = register_a / 2usize.pow(register_b as u32);
        register_a /= 8;
        register_b ^= register_c;
        register_b ^= 5;
        stdout.push(register_b % 8);
        if register_a == 0 {
            break;
        }
    }
    stdout
}

pub fn run() -> String {
    let filename = "inputs/day17.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let computer = Computer::from(&content);
    let program = parse_program(&content);
    dbg!(&program);
    let output = run_decompiled_program();
    output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
    // computer.copy_program(&program).unwrap_or(-1isize)
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_step_forward() {
//         assert_eq!(1, 1);
//     }
// }
