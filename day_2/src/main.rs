use std::fs::File;
use std::io::Read;

fn main() {
    day_2_part_1();
    day_2_part_2();
}

fn day_2_part_1() {
    let mut file = File::open("input.txt").expect("Can't find input file.");
    let mut buffer = String::new();

    let _ = file.read_to_string(&mut buffer);

    let mut acc = 0;

    for line in buffer.lines() {
        let split_line = line.split_whitespace();

        let max_num = split_line
            .clone()
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|n| n % 2 == 0)
            .max()
            .unwrap();

        let min_num = split_line
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|n| n % 2 == 0)
            .min()
            .unwrap();

        acc += max_num / min_num;
    }

    println!("Checksum: {}", acc);
}

fn day_2_part_2() {
    let mut file = File::open("input.txt").expect("Can't find input file.");
    let mut buffer = String::new();

    let _ = file.read_to_string(&mut buffer);

    let mut acc = 0;

    for line in buffer.lines() {
        let split_line = line.split_whitespace();

        let nums = split_line
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut div = 0;

        'ol: for num in &nums {
            for num2 in &nums {
                if num != num2 && num % num2 == 0 {
                    div = num / num2;
                    break 'ol;
                } else if num != num2 && num2 % num == 0 {
                    div = num2 / num;
                    break 'ol;
                }
            }
        }

        acc += div;
    }

    println!("Checksum: {}", acc);
}
