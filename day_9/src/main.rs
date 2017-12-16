
fn main() {
    let input = include_str!("../input.txt");

    println!("Total score: {}", day_9_part_1(input));
    println!("Total garbage characters: {}", day_9_part_2(input));
}

fn day_9_part_1(input: &str) -> usize {
    let mut chars = input.chars();

    parse(&mut chars, 0)
}

fn parse<I: Iterator<Item = char>>(iter: &mut I, level: usize) -> usize {
    let mut count = level;

    while let Some(c) = iter.next() {
        if c == '}' {
            break;
        }

        match c {
            '<' => parse_garbage(iter),
            '!' => {
                let _ = iter.next();
            }
            '{' => count += parse(iter, level + 1),
            _ => {}
        }
    }

    count
}


fn parse_garbage<I: Iterator<Item = char>>(iter: &mut I) {
    while let Some(c) = iter.next() {
        if c == '>' {
            break;
        }

        if c == '!' {
            let _ = iter.next();
        }
    }
}

fn day_9_part_2(input: &str) -> usize {
    let mut count = 0;

    let mut iter = input.chars();
    let mut inside_garbage = false;

    while let Some(c) = iter.next() {
        if !inside_garbage && c == '<' {
            inside_garbage = true;
        } else if inside_garbage {
            if c == '>' {
                inside_garbage = false;
            } else if c == '!' {
                let _ = iter.next();
            } else {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn example_1() {
    let input = "{}";
    let result = day_9_part_1(input);

    assert_eq!(result, 1);
}

#[test]
fn example_2() {
    let input = "{{{}}}";
    let result = day_9_part_1(input);

    assert_eq!(result, 6);
}

#[test]
fn example_3() {
    let input = "{{},{}}";
    let result = day_9_part_1(input);

    assert_eq!(result, 5);
}

#[test]
fn example_4() {
    let input = "{{{},{},{{}}}}}";
    let result = day_9_part_1(input);

    assert_eq!(result, 16);
}

#[test]
fn example_5() {
    let input = "{<a>,<a>,<a>,<a>}";
    let result = day_9_part_1(input);

    assert_eq!(result, 1);
}

#[test]
fn example_6() {
    let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
    let result = day_9_part_1(input);

    assert_eq!(result, 9);
}

#[test]
fn example_7() {
    let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
    let result = day_9_part_1(input);

    assert_eq!(result, 9);
}

#[test]
fn example_8() {
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    let result = day_9_part_1(input);

    assert_eq!(result, 3);
}

#[test]
fn example_1_part_2() {
    let input = "<>";
    let result = day_9_part_2(input);

    assert_eq!(result, 0);
}

#[test]
fn example_2_part_2() {
    let input = "<random characters>";
    let result = day_9_part_2(input);

    assert_eq!(result, 17);
}

#[test]
fn example_3_part_2() {
    let input = "<<<<>";
    let result = day_9_part_2(input);

    assert_eq!(result, 3);
}

#[test]
fn example_4_part_2() {
    let input = "<{!>}>>";
    let result = day_9_part_2(input);

    assert_eq!(result, 2);
}

#[test]
fn example_5_part_2() {
    let input = "<!!>";
    let result = day_9_part_2(input);

    assert_eq!(result, 0);
}

#[test]
fn example_6_part_2() {
    let input = "<!!!>>";
    let result = day_9_part_2(input);

    assert_eq!(result, 0);
}

#[test]
fn example_7_part_2() {
    let input = "<{o\"i!a,<{i<a>";
    let result = day_9_part_2(input);

    assert_eq!(result, 10);
}
