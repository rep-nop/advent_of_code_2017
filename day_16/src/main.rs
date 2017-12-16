#![feature(slice_rotate)]

fn main() {
    let input = include_str!("../input.txt");
    let programs = vec![
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
        'i',
        'j',
        'k',
        'l',
        'm',
        'n',
        'o',
        'p',
    ];

    println!("Part 1: {}", day_16_part_1(input, programs.clone()));
    println!("Part 2: {}", day_16_part_2(input, programs));
}

// Very crude implementation of the state machine
fn day_16_part_1(input: &str, mut programs: Vec<char>) -> String {
    let mut output = String::with_capacity(16);

    let operations = input.split(',').collect::<Vec<&str>>();

    // Perform the next operation on the programs
    for operation in operations {
        let op_code = operation.chars().nth(0).unwrap();
        match op_code {
            // Rotate ("shift") the values n steps to the right
            's' => {
                let rot_amt = operation.split_at(1).1.parse::<usize>().unwrap();
                let len = programs.len();

                // `len - rot_amt` allows us to rotate right instead of left
                programs.rotate(len - rot_amt);
            }
            // Exchange two programs based on position
            'x' => {
                let mut positions = operation.trim_matches('x').split('/');
                let fpos = positions.next().unwrap().parse().unwrap();
                let spos = positions.next().unwrap().parse().unwrap();

                programs.swap(fpos, spos);
            }
            // Same as "x" except they force us to find the positions of the specified
            //  programs first :P
            'p' => {
                let mut programs_to_find = operation.split_at(1).1.split('/');
                let fprog = programs_to_find.next().unwrap().chars().nth(0).unwrap();
                let sprog = programs_to_find.next().unwrap().chars().nth(0).unwrap();

                let fpos = programs.iter().position(|&c| c == fprog).unwrap();
                let spos = programs.iter().position(|&c| c == sprog).unwrap();

                programs.swap(fpos, spos);
            }
            _ => unreachable!(),
        }
    }

    programs.iter().for_each(|&c| output.push(c));

    output
}

// Obviously with 1 ***billion*** iterations of each operation
//  there's a trick to it, the most likely candidate? Cycles.
// So find how many iterations it takes to get us back to the original
//  starting state, then perform the amount extra we need to get the
//  correct output
fn day_16_part_2(input: &str, mut programs: Vec<char>) -> String {
    let mut output = String::with_capacity(16);

    let operations = input.split(',').collect::<Vec<&str>>();

    let mut counter = 0;

    let orig_programs = programs.clone();

    loop {
        for operation in &operations {
            let op_code = operation.chars().nth(0).unwrap();
            match op_code {
                's' => {
                    let rot_amt = operation.split_at(1).1.parse::<usize>().unwrap();
                    let len = programs.len();
                    programs.rotate(len - rot_amt);
                }
                'x' => {
                    let mut positions = operation.trim_matches('x').split('/');
                    let fpos = positions.next().unwrap().parse().unwrap();
                    let spos = positions.next().unwrap().parse().unwrap();

                    programs.swap(fpos, spos);
                }
                'p' => {
                    let mut programs_to_find = operation.split_at(1).1.split('/');
                    let fprog = programs_to_find.next().unwrap().chars().nth(0).unwrap();
                    let sprog = programs_to_find.next().unwrap().chars().nth(0).unwrap();

                    let fpos = programs.iter().position(|&c| c == fprog).unwrap();
                    let spos = programs.iter().position(|&c| c == sprog).unwrap();

                    programs.swap(fpos, spos);
                }
                _ => unreachable!(),
            }
        }

        counter += 1;

        if programs == orig_programs {
            break;
        }
    }

    for _ in 0..(1_000_000_000 % counter) {
        for operation in &operations {
            let op_code = operation.chars().nth(0).unwrap();
            match op_code {
                's' => {
                    let rot_amt = operation.split_at(1).1.parse::<usize>().unwrap();
                    let len = programs.len();
                    programs.rotate(len - rot_amt);
                }
                'x' => {
                    let mut positions = operation.trim_matches('x').split('/');
                    let fpos = positions.next().unwrap().parse().unwrap();
                    let spos = positions.next().unwrap().parse().unwrap();

                    programs.swap(fpos, spos);
                }
                'p' => {
                    let mut programs_to_find = operation.split_at(1).1.split('/');
                    let fprog = programs_to_find.next().unwrap().chars().nth(0).unwrap();
                    let sprog = programs_to_find.next().unwrap().chars().nth(0).unwrap();

                    let fpos = programs.iter().position(|&c| c == fprog).unwrap();
                    let spos = programs.iter().position(|&c| c == sprog).unwrap();

                    programs.swap(fpos, spos);
                }
                _ => unreachable!(),
            }
        }
    }

    programs.iter().for_each(|&c| output.push(c));

    output
}

#[test]
fn example_1() {
    let input = "s1,x3/4,pe/b";
    let result = day_16_part_1(input, vec!['a', 'b', 'c', 'd', 'e']);

    assert_eq!(result, "baedc");
}
