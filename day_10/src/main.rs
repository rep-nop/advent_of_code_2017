fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day_10_part_1(input));
    day_10_part_2(input);
}

fn day_10_part_1(input: &str) -> usize {
    let mut list = (0..256).collect::<Vec<usize>>();
    let mut skip_size = 0;
    let mut curr_pos = 0;
    let input_lengths = input
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<usize>>();

    for length in input_lengths {
        reverse_slice_section_circular(&mut list[..], curr_pos % 256, length);
        curr_pos += skip_size + length;
        skip_size += 1;
    }

    list[0] * list[1]
}

fn day_10_part_2(input: &str) {
    let mut list = (0..256).collect::<Vec<usize>>();
    let mut skip_size = 0;
    let mut curr_pos = 0;
    let mut input_lengths = input.chars().map(|i| i as usize).collect::<Vec<usize>>();

    println!("{:?}", input_lengths);

    input_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    for _ in 0..64 {
        for &length in &input_lengths {
            reverse_slice_section_circular(&mut list[..], curr_pos % 256, length);
            curr_pos += skip_size + length;
            skip_size += 1;
        }
    }

    let mut collection = Vec::with_capacity(16);

    for i in 0..16 {
        collection.push(
            (list[i * 16 + 0] ^ list[i * 16 + 1] ^ list[i * 16 + 2] ^ list[i * 16 + 3]
                ^ list[i * 16 + 4] ^ list[i * 16 + 5] ^ list[i * 16 + 6]
                ^ list[i * 16 + 7] ^ list[i * 16 + 8] ^ list[i * 16 + 9]
                ^ list[i * 16 + 10] ^ list[i * 16 + 11] ^ list[i * 16 + 12]
                ^ list[i * 16 + 13] ^ list[i * 16 + 14] ^ list[i * 16 + 15]) as u8,
        );
    }

    println!("{}", collection.len());

    for val in collection {
        print!("{:0>2x}", val);
    }

    println!("");
}

fn reverse_slice_section_circular(slice: &mut [usize], start: usize, len: usize) {
    let mut rev = slice
        .iter()
        .cycle()
        .skip(start)
        .take(len)
        .map(|x| *x)
        .collect::<Vec<usize>>();

    rev.reverse();

    for (i, &v) in rev.iter().enumerate() {
        let idx = if start + i >= slice.len() {
            (start + i) - slice.len()
        } else {
            start + i
        };

        slice[idx] = v;
    }
}

#[test]
fn test_swap_normal() {
    let mut arr = [0, 1, 2, 3, 4];
    reverse_slice_section_circular(&mut arr[..], 0, 5);
    assert_eq!(arr, [4, 3, 2, 1, 0]);
}

#[test]
fn test_swap_start_bigger() {
    let mut arr = [2, 1, 0, 3, 4];
    reverse_slice_section_circular(&mut arr[..], 3, 4);
    assert_eq!(arr, [4, 3, 0, 1, 2]);
}
