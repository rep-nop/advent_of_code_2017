use std::collections::HashMap;

// Following enums/structs & their methods should be fairly
// self-explanitory
enum Operation {
    Increment(isize),
    Decrement(isize),
}

impl Operation {
    fn apply(&self, val: &mut isize) -> isize {
        use Operation::*;

        match *self {
            Increment(i) => {
                *val += i;
                *val
            }
            Decrement(i) => {
                *val -= i;
                *val
            }
        }
    }
}

enum Comparison {
    Equal(isize),             // ==
    NotEqual(isize),          // !=
    LessThan(isize),          // <
    LessThanEqual(isize),     // <=
    GreaterThan(isize),       // >
    GreatherThanEqual(isize), // >=
}

impl Comparison {
    fn apply(&self, lhs: isize) -> bool {
        use Comparison::*;

        match *self {
            Equal(i) => lhs == i,
            NotEqual(i) => lhs != i,
            LessThan(i) => lhs < i,
            LessThanEqual(i) => lhs <= i,
            GreaterThan(i) => lhs > i,
            GreatherThanEqual(i) => lhs >= i,
        }
    }
}

struct Instruction {
    reg_store: String,
    op: Operation,
    compare: Comparison,
    reg_get: String,
}

impl<'a> From<&'a str> for Instruction {
    fn from(line: &'a str) -> Instruction {
        let components = line.split_whitespace().collect::<Vec<_>>();

        let change_by = components[2].parse().unwrap();

        let op = if components[1] == "inc" {
            Operation::Increment(change_by)
        } else {
            Operation::Decrement(change_by)
        };

        let compare_to = components[6].parse().unwrap();

        let compare = match components[5] {
            "==" => Comparison::Equal(compare_to),
            "!=" => Comparison::NotEqual(compare_to),
            "<" => Comparison::LessThan(compare_to),
            "<=" => Comparison::LessThanEqual(compare_to),
            ">" => Comparison::GreaterThan(compare_to),
            ">=" => Comparison::GreatherThanEqual(compare_to),
            _ => unreachable!(),
        };

        let reg_store = components[0].to_string();

        let reg_get = components[4].to_string();

        Instruction {
            reg_store: reg_store,
            op: op,
            compare: compare,
            reg_get: reg_get,
        }
    }
}

#[derive(Default)]
struct Machine {
    instructions: Vec<Instruction>,
    registers: HashMap<String, isize>,
}

impl Machine {
    // Returns the greatest value encountered at any point
    fn run(&mut self) -> isize {
        let mut largest = 0;
        let iter = self.instructions.iter();

        for instruction in iter {
            // Gets the value of the register we need to compare
            let reg_get = {
                *self.registers
                    .entry(instruction.reg_get.clone())
                    .or_insert_with(|| 0isize)
            };

            // Get a mutable reference to the register we're modifying
            let reg_store = self.registers
                .entry(instruction.reg_store.clone())
                .or_insert_with(|| 0isize);

            if instruction.compare.apply(reg_get) {
                let result = instruction.op.apply(reg_store);

                if result > largest {
                    largest = result;
                }
            }
        }

        largest
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (largest_at_end, largest_any) = day_8(input);
    println!("largest value at end: {}", largest_at_end);
    println!("largest value at any time: {}", largest_any);
}

// Performs both parts of day 8 for ease of use
fn day_8(input: &str) -> (isize, isize) {
    let mut machine = Machine::default();

    input
        .lines()
        .map(|l| l.into())
        .for_each(|i| machine.instructions.push(i));

    let result = machine.run();

    (
        machine.registers.iter().map(|(_, &i)| i).max().unwrap(),
        result,
    )
}

#[test]
fn example_1() {
    let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";

    let result = day_8(input);

    assert_eq!(result, (1, 10));
}
