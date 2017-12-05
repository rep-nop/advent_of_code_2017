fn main() {
    let input = include_str!("..//input.txt");

    println!("Part 1: {} steps", day_5_part_1(input));
    println!("Part 2: {} steps", day_5_part_2(input));
}

fn day_5_part_1(input: &str) -> usize {
    let mut count = 0;
    let mut index = 0;

    // Convert the input into a vector of i64s
    let mut jumps = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    // Loop infinitely until we find an index that would be out of bounds
    loop {
        count += 1;

        // Get the offset
        let jump_val = jumps[index];

        // Increment the offset since we've processed it
        jumps[index] += 1;

        // Calculate the new index
        let new_index = index as i64 + jump_val;

        // If we're going to go out of bounds, we've escaped the "maze"
        if new_index < 0 || new_index > (jumps.len() - 1) as i64 {
            break;
        }

        // Otherwise make it the next index
        index = new_index as usize;
    }

    count
}

fn day_5_part_2(input: &str) -> usize {
    let mut count = 0;
    let mut index = 0;

    // Convert the input into a vector of i64s
    let mut jumps = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    // Loop infinitely until we find an index that would be out of bounds
    loop {
        count += 1;

        // Get the offset
        let jump_val = jumps[index];

        // Increment or decrement (if offset >= 3) the offset
        // since we've processed it
        if jump_val >= 3 {
            jumps[index] -= 1;
        } else {
            jumps[index] += 1;
        }

        // Calculate the new index
        let new_index = index as i64 + jump_val;

        // If we're going to go out of bounds, we've escaped the "maze"
        if new_index < 0 || new_index > (jumps.len() - 1) as i64 {
            break;
        }

        // Otherwise make it the next index
        index = new_index as usize;
    }

    count
}

#[test]
fn example_1() {
    let input = "0\n3\n0\n1\n-3";
    let result = day_5_part_1(input);

    assert_eq!(result, 5);
}

#[test]
fn example_1_part_2() {
    let input = "0\n3\n0\n1\n-3";
    let result = day_5_part_2(input);

    assert_eq!(result, 10);
}
