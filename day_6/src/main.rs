use std::collections::HashSet;

fn main() {
    let input = include_str!("..//input.txt");
    println!("{} steps", day_6_part_1(input).0);
    println!("{} steps", day_6_part_2(input));
}

// Returns the ending Vec for ease of reuse in Part 2
fn day_6_part_1(input: &str) -> (usize, Vec<usize>) {
    let mut count = 0;

    // Make a Vec out of the input
    let mut banks = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    // Easiest way to check for dupes is to use a HashSet
    let mut patterns = HashSet::new();

    // `insert` returns a bool
    //      true if the value didn't match one already in
    //      false if it already existed
    while patterns.insert(banks.clone()) {
        // Get the index and value of the block to redistribute
        let (mut max_blocks, index) = {
            let mut iter = banks.iter();
            let max_blocks = iter.clone().max().unwrap();
            let index = iter.position(|&x| x == *max_blocks).unwrap();
            (*max_blocks, index)
        };

        // Set that index to 0 since we're redistributing
        banks[index] = 0;

        // Make sure the index we start at isn't past the end
        let mut start_fill = if index + 1 > banks.len() - 1 {
            0
        } else {
            index + 1
        };

        // Redistribute one block to every bank until we don't have any more
        while max_blocks > 0 {
            banks[start_fill] += 1;
            start_fill += 1;

            if start_fill > banks.len() - 1 {
                start_fill = 0;
            }

            max_blocks -= 1;
        }

        count += 1;
    }


    // Return the number of cycles (and the ending Vec)
    (count, banks)
}

// Part two requires us to use the ending state of the Vec
// from part one and find the number of cycles until it
// appears again
fn day_6_part_2(input: &str) -> usize {
    let mut count = 0;

    // Get the ending Vec state from part one
    let banks_origin = day_6_part_1(input).1;
    // .. and make a copy of it to modify
    let mut banks = banks_origin.clone();

    // We use a loop to ensure the other Vec is modified first
    // before comparing
    loop {
        // Otherwise follows pretty much the same algorithm as part one
        let (mut max_blocks, index) = {
            let mut iter = banks.iter();
            let max_blocks = iter.clone().max().unwrap();
            let index = iter.position(|&x| x == *max_blocks).unwrap();
            (*max_blocks, index)
        };

        banks[index] = 0;

        let mut start_fill = if index + 1 > banks.len() - 1 {
            0
        } else {
            index + 1
        };

        while max_blocks > 0 {
            banks[start_fill] += 1;
            start_fill += 1;

            if start_fill > banks.len() - 1 {
                start_fill = 0;
            }

            max_blocks -= 1;
        }

        count += 1;

        // Just break out if we've found it again
        if banks == banks_origin {
            break;
        }
    }

    count
}

#[test]
fn example_1() {
    let input = "0 2 7 0";
    let result = day_6_part_1(input);

    assert_eq!(result.0, 5);
}

#[test]
fn example_1_part_2() {
    let input = "0 2 7 0";
    let result = day_6_part_2(input);

    assert_eq!(result, 4);
}
