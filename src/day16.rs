use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
struct RegisterIndex(usize);

type RegisterType = i64;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Opcode {
    Addr,
    Addi,

    Mulr,
    Muli,

    Banr,
    Bani,

    Borr,
    Bori,

    Setr,
    Seti,

    Gtir,
    Gtri,
    Gtrr,

    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug)]
enum Value {
    Immediate(RegisterType),
    FromRegister(RegisterIndex),
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    first_operand: Value,
    second_operand: Option<Value>,
    destination: RegisterIndex,
}

lazy_static! {
    static ref OPCODE_MAPPINGS: HashMap<Opcode, (bool, bool)> = vec![
        (Opcode::Addr, true, true),
        (Opcode::Addi, true, false),
        (Opcode::Mulr, true, true),
        (Opcode::Muli, true, false),
        (Opcode::Banr, true, true),
        (Opcode::Bani, true, false),
        (Opcode::Borr, true, true),
        (Opcode::Bori, true, false),
        (Opcode::Setr, true, false),
        (Opcode::Seti, false, false),
        (Opcode::Gtir, false, true),
        (Opcode::Gtri, true, false),
        (Opcode::Gtrr, true, true),
        (Opcode::Eqir, false, true),
        (Opcode::Eqri, true, false),
        (Opcode::Eqrr, true, true),
    ].into_iter()
    .map(|(opcode, first_is_ref, second_is_ref)| (opcode, (first_is_ref, second_is_ref)))
    .collect();
}

impl Instruction {
    fn new(
        opcode: Opcode,
        first_operand: usize,
        second_operand: usize,
        destination: usize,
    ) -> Self {
        let (first_is_ref, second_is_ref) = OPCODE_MAPPINGS.get(&opcode).unwrap();

        Self {
            opcode,
            first_operand: if *first_is_ref {
                Value::FromRegister(RegisterIndex(first_operand))
            } else {
                Value::Immediate(first_operand as i64)
            },
            second_operand: if *second_is_ref {
                Some(Value::FromRegister(RegisterIndex(second_operand)))
            } else {
                Some(Value::Immediate(second_operand as i64))
            },
            destination: RegisterIndex(destination),
        }
    }

    fn from_potential_instruction(instruction: &[usize]) -> Vec<Self> {
        vec![
            (Opcode::Addr, true, true),
            (Opcode::Addi, true, false),
            (Opcode::Mulr, true, true),
            (Opcode::Muli, true, false),
            (Opcode::Banr, true, true),
            (Opcode::Bani, true, false),
            (Opcode::Borr, true, true),
            (Opcode::Bori, true, false),
            (Opcode::Setr, true, false),
            (Opcode::Seti, false, false),
            (Opcode::Gtir, false, true),
            (Opcode::Gtri, true, false),
            (Opcode::Gtrr, true, true),
            (Opcode::Eqir, false, true),
            (Opcode::Eqri, true, false),
            (Opcode::Eqrr, true, true),
        ].into_iter()
        .map(|(opcode, first_is_ref, second_is_ref)| Self {
            opcode,
            first_operand: if first_is_ref {
                Value::FromRegister(RegisterIndex(instruction[1]))
            } else {
                Value::Immediate(instruction[1] as i64)
            },
            second_operand: if second_is_ref {
                Some(Value::FromRegister(RegisterIndex(instruction[2])))
            } else {
                Some(Value::Immediate(instruction[2] as i64))
            },
            destination: RegisterIndex(instruction[3]),
        }).collect()
    }
}

struct Machine {
    registers: [RegisterType; 4],
}

impl Machine {
    fn new() -> Self {
        Self { registers: [0; 4] }
    }

    fn set_register_state(&mut self, values: &[RegisterType]) {
        assert!(
            values.len() == 4,
            "Cannot set registers unless length matches"
        );

        self.registers[0] = values[0];
        self.registers[1] = values[1];
        self.registers[2] = values[2];
        self.registers[3] = values[3];
    }

    fn execute(&mut self, instruction: &Instruction) {
        let a = self.get_value(&instruction.first_operand);
        let b = instruction
            .second_operand
            .as_ref()
            .map(|value| self.get_value(&value));
        let c = instruction.destination;

        match instruction.opcode {
            Opcode::Addr | Opcode::Addi => self[c] = a + b.unwrap(),

            Opcode::Mulr | Opcode::Muli => self[c] = a * b.unwrap(),

            Opcode::Banr | Opcode::Bani => self[c] = a & b.unwrap(),

            Opcode::Borr | Opcode::Bori => self[c] = a | b.unwrap(),

            Opcode::Setr | Opcode::Seti => self[c] = a,

            Opcode::Gtir | Opcode::Gtri | Opcode::Gtrr => {
                self[c] = if a > b.unwrap() { 1 } else { 0 }
            }

            Opcode::Eqir | Opcode::Eqri | Opcode::Eqrr => {
                self[c] = if a == b.unwrap() { 1 } else { 0 }
            }
        }
    }

    fn get_value(&self, value: &Value) -> RegisterType {
        match value {
            Value::Immediate(v) => v.clone(),
            Value::FromRegister(idx) => self[*idx],
        }
    }
}

impl Index<RegisterIndex> for Machine {
    type Output = RegisterType;

    fn index(&self, register_index: RegisterIndex) -> &RegisterType {
        &self.registers[register_index.0]
    }
}

impl IndexMut<RegisterIndex> for Machine {
    fn index_mut(&mut self, register_index: RegisterIndex) -> &mut RegisterType {
        &mut self.registers[register_index.0]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Observation {
    before: Vec<RegisterType>,
    instruction: Vec<usize>,
    after: Vec<RegisterType>,
}

impl Observation {
    fn new(before: Vec<RegisterType>, instruction: Vec<usize>, after: Vec<RegisterType>) -> Self {
        Self {
            before,
            instruction,
            after,
        }
    }
}

impl<'a> From<&'a [&'a str]> for Observation {
    fn from(input: &'a [&'a str]) -> Self {
        let iterator = input
            .iter()
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .enumerate();

        let parsed_expectations: Vec<Vec<RegisterType>> = iterator
            .clone()
            .filter(|(id, _)| id == &0 || id == &2)
            .map(|(_, line)| {
                line.replace("Before:", "")
                    .replace("After: ", "")
                    .replace("[", "")
                    .replace("]", "")
                    .replace(",", "")
                    .split(" ")
                    .filter(|s| s.len() > 0)
                    .map(|s| {
                        s.trim().parse::<RegisterType>().expect(&format!(
                            "Expected valid input when parsing before/after 🤷‍♂️ in `{}`",
                            s
                        ))
                    }).collect()
            }).collect();

        let parsed_instructions: Vec<usize> = iterator
            .filter(|(id, _)| id == &1)
            .flat_map(|(_, line)| {
                line.split(" ").map(|s| {
                    s.trim()
                        .parse::<usize>()
                        .expect(&format!("Expected valid input 🤷‍♂️ in `{}`", s))
                })
            }).collect();

        assert!(parsed_expectations.len() == 2, "Invalid input {:?}", input);
        assert!(
            parsed_expectations[0].len() == 4,
            "Invalid input {:?}",
            input
        );
        assert!(
            parsed_expectations[1].len() == 4,
            "Invalid input {:?}",
            input
        );
        assert!(parsed_instructions.len() == 4, "Invalid input {:?}", input);

        Self {
            before: parsed_expectations[0].clone(),
            instruction: parsed_instructions,
            after: parsed_expectations[1].clone(),
        }
    }
}

pub fn star_one(input: &str) -> i64 {
    let cleaned_lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();
    let observations = cleaned_lines
        .chunks(3)
        .map(|chunk| Observation::from(chunk))
        .collect::<Vec<_>>();

    observations
        .into_iter()
        .map(|observation| {
            let potential_instructions =
                Instruction::from_potential_instruction(&observation.instruction);

            potential_instructions
                .into_iter()
                .flat_map(|instruction| {
                    let mut machine = Machine::new();
                    machine.set_register_state(&observation.before);

                    machine.execute(&instruction);

                    if machine.registers[..] == observation.after[..] {
                        Some(instruction)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
        }).fold(
            0,
            |acc, matched_ops| if matched_ops.len() >= 3 { acc + 1 } else { acc },
        )
}

pub fn star_two(observations: &str, program_source: &str) -> i64 {
    let cleaned_lines = observations
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();
    let observations = cleaned_lines
        .chunks(3)
        .map(|chunk| Observation::from(chunk))
        .collect::<Vec<_>>();

    let mut observed_opcodes: HashMap<usize, HashSet<Opcode>> = HashMap::new();

    observations.into_iter().for_each(|observation| {
        let potential_instructions =
            Instruction::from_potential_instruction(&observation.instruction);

        potential_instructions.into_iter().for_each(|instruction| {
            let mut machine = Machine::new();
            machine.set_register_state(&observation.before);

            machine.execute(&instruction);

            if machine.registers[..] == observation.after[..] {
                let entry = observed_opcodes
                    .entry(observation.instruction[0])
                    .or_insert(HashSet::new());
                entry.insert(instruction.opcode);
            }
        });
    });

    let mut mappings = HashMap::<usize, Opcode>::new();

    while !observed_opcodes.is_empty() {
        let (opcode_number, current_opcode) = {
            let (opcode_number, opcodes): (usize, HashSet<Opcode>) = observed_opcodes
                .iter()
                .filter(|(_, opcodes)| opcodes.len() == 1)
                .map(|(number, opcodes)| (number.clone(), opcodes.clone()))
                .nth(0)
                .expect(&format!(
                "There should always be an Opcode that only maps to a single opcode number.\n {:?}",
                observed_opcodes
            ));
            let current_opcode = opcodes.iter().nth(0).map(|code| code.clone()).unwrap();

            (opcode_number, current_opcode)
        };

        mappings.insert(opcode_number.clone(), current_opcode);
        observed_opcodes.remove(&opcode_number);

        {
            for (_, opcodes) in observed_opcodes.iter_mut() {
                opcodes.remove(&current_opcode);
            }
        }
    }

    let instructions = program_source
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| {
            let instruction = line
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Instruction::new(
                mappings[&instruction[0]],
                instruction[1],
                instruction[2],
                instruction[3],
            )
        });
    let mut machine = Machine::new();

    for instruction in instructions {
        machine.execute(&instruction);
    }

    machine.registers[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observation_from() {
        let input = ["Before: [3, 3, 0, 2]", "10 2 0 1", "After:  [3, 0, 0, 2]"];
        let observation = Observation::from(&input[..]);
    }
}
