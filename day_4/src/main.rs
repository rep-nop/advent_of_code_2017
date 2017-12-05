use std::collections::HashSet;
use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").expect("Can't find input");
    let mut contents = String::new();

    let _ = f.read_to_string(&mut contents);

    println!("{}", day_1_part_1(&contents));
    println!("{}", day_1_part_2(&contents));
}

fn day_1_part_1(input: &str) -> u32 {
    let mut count = 0;

    let passphrases = input.lines();

    'l: for passphase in passphrases {
        let mut hs = HashSet::new();

        for word in passphase.split(' ') {
            match hs.get(&word) {
                Some(_) => continue 'l,
                None => hs.insert(word),
            };
        }

        count += 1;
    }

    count
}

fn day_1_part_2(input: &str) -> u32 {
    let mut count = 0;

    let passphrases = input.lines();

    'l: for passphase in passphrases {
        let mut hs = HashSet::new();

        for word in passphase.split(' ') {
            let mut v = word.chars().collect::<Vec<_>>();

            v.sort_unstable();

            match hs.get(&v) {
                Some(_) => continue 'l,
                None => hs.insert(v),
            };
        }

        count += 1;
    }

    count
}

#[test]
fn example_1() {
    let input = "aa bb cc dd ee";
    let result = day_1_part_1(input);

    assert_eq!(result, 1);
}

#[test]
fn example_2() {
    let input = "aa bb cc dd ee aa";
    let result = day_1_part_1(input);

    assert_eq!(result, 0);
}

#[test]
fn example_3() {
    let input = "aa bb cc dd ee aaa";
    let result = day_1_part_1(input);

    assert_eq!(result, 1);
}

#[test]
fn example_1_part_2() {
    let input = "aa bb cc dd ee";
    let result = day_1_part_2(input);

    assert_eq!(result, 1);
}

#[test]
fn example_2_part_2() {
    let input = "ab bb cc dd ee ba";
    let result = day_1_part_2(input);

    assert_eq!(result, 0);
}

#[test]
fn example_3_part_2() {
    let input = "aa bb cc dd ee aaa";
    let result = day_1_part_2(input);

    assert_eq!(result, 1);
}
