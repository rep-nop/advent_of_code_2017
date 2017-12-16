fn main() {
    println!("Judge count (part 1): {}", day_15_part_1());
    println!("Judge count (part 2): {}", day_15_part_2());
}

fn day_15_part_1() -> usize {
    const A_FACTOR: usize = 16807;
    const B_FACTOR: usize = 48271;

    let mut eq_count = 0;

    // Uses *_FACTOR and the previous value mod 2147483647 to calculate the next value
    let gen_a = |prev: usize| (prev * A_FACTOR) % 2147483647;
    let gen_b = |prev: usize| (prev * B_FACTOR) % 2147483647;

    // Initial values
    let mut prev_a = 783;
    let mut prev_b = 325;

    // Compare the lower 16 bits of 40 million pairs
    for _ in 0..40_000_000 {
        prev_a = gen_a(prev_a);
        prev_b = gen_b(prev_b);

        if prev_a & 0xFFFF == prev_b & 0xFFFF {
            eq_count += 1;
        }
    }

    eq_count
}

fn day_15_part_2() -> usize {
    const A_FACTOR: usize = 16807;
    const B_FACTOR: usize = 48271;

    let mut eq_count = 0;

    let gen_a = |prev: usize| (prev * A_FACTOR) % 2147483647;
    let gen_b = |prev: usize| (prev * B_FACTOR) % 2147483647;

    let mut prev_a = 783;
    let mut prev_b = 325;

    for _ in 0..5_000_000 {

        // For both generators A and B, we need to get a pair of values
        //  that satisfies value_A % 4 == 0 and value_B % 8 == 0

        // Need a post-test loop so it doesn't get stuck on one value
        // May or may not have realized that later than I should have
        loop {
            prev_a = gen_a(prev_a);

            if prev_a % 4 == 0 {
                break;
            }
        }

        loop {
            prev_b = gen_b(prev_b);

            if prev_b % 8 == 0 {
                break;
            }
        }

        if prev_a & 0xFFFF == prev_b & 0xFFFF {
            eq_count += 1;
        }
    }

    eq_count
}
