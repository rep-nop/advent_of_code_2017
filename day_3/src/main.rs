use std::collections::HashMap;

fn main() {
    let (x, y) = day_3_part_1(347991);
    println!("{}", x.abs() + y.abs());
    day_3_part_2(347991);
}

// Beware, math ahead.
//  Abandon all hope ye who enter here.


// Calculates the coordinates for any number in the spiral
//
//  17  16  15  14  13
//  18   5   4   3  12
//  19   6   1   2  11
//  20   7   8   9  10
//  21  22  23  24  25 ->
//
//  How it works:
//
//  The diagnal leading south-east from the center (1)
//  always contains the square of the next odd number
//  starting from 3 (3^2 = 9, 5^2 = 25, etc)
//  and the coordinates of them can be found by the forumla
//      let square_root = odd_square.sqrt();
//      let x_coord = square_root - ((square_root - 1) / 2 + 1);
//      let y_coord = -x;
//
//  This forumla essentially maps the odd integers to the set of
//  natural numbers which allows us to explicity define the
//  coordinates for the squares.
//
//  Using this knowledge, if we can know which "side" the number
//  we're looking for resides on, we can figure out either its
//  x or y coordinate immediately based on the next odd sqaure.
//
//  The "sides" defined by this algorithm are as follows:
//      [1] [1] [1] [1]
//      17  16  15  14  13 [0]
//  [2] 18              12 [0]
//  [2] 19              11 [0]
//  [2] 20              10 [0]
//  [2] 21  22  23  24  25
//         [3] [3] [3] [3]
//
//  (technically 25 would be considered "4", but that case is
//  handled easily)
//
//  So to find the side we're working with, we first normalize
//  the ... I've forgotten what else to write and am tired,
//  you get the gist. ayy lmao
fn day_3_part_1(n: u64) -> (i64, i64) {
    let n = n as f64;
    let p = n.sqrt().ceil();
    let next_odd_square = if p as u64 % 2 == 0 {
        (p + 1.0) * (p + 1.0)
    } else {
        p * p
    };

    let last_odd_square = (next_odd_square.sqrt() - 2.0) * (next_odd_square.sqrt() - 2.0);

    let nums_in_spiral = next_odd_square - (last_odd_square + 1.0);

    let side = ((n - (last_odd_square + 1.0))
        / ((next_odd_square - (last_odd_square + 1.0)) * 0.25))
        .floor();

    let side_x_end = |x: f64| ((last_odd_square + 1.0) + (x * 0.25 * nums_in_spiral)).floor();

    let (x, y) = match side as i64 {
        0 => {
            let sqt = next_odd_square.sqrt() as i64;
            let (mut coordx, mut coordy) = ((sqt - ((sqt - 1) / 2 + 1)), 0);

            let diff = side_x_end(1 as f64) as i64 - (n as i64);
            coordy = coordx - diff;
            (coordx, coordy)
        }
        1 => {
            let sqt = next_odd_square.sqrt() as i64;
            let (mut coordx, mut coordy) = (0, (sqt - ((sqt - 1) / 2 + 1)));

            let diff = side_x_end(2 as f64) as i64 - (n as i64);
            coordx = -coordy + diff;
            (coordx, coordy)
        }
        2 => {
            let sqt = next_odd_square.sqrt() as i64;
            let (mut coordx, mut coordy) = (-(sqt - ((sqt - 1) / 2 + 1)), 0);

            let diff = side_x_end(3 as f64) as i64 - (n as i64);
            coordy = coordx + diff;
            (coordx, coordy)
        }
        3 => {
            let sqt = next_odd_square.sqrt() as i64;
            let (mut coordx, mut coordy) = (0, -(sqt - ((sqt - 1) / 2 + 1)));

            let diff = side_x_end(4 as f64) as i64 - (n as i64);
            coordx = -coordy - diff;
            (coordx, coordy)
        }
        _ => {
            let sqt = next_odd_square.sqrt() as i64;
            ((sqt - ((sqt - 1) / 2 + 1)), -(sqt - ((sqt - 1) / 2 + 1)))
        }
    };

    (x, y)
}

fn day_3_part_2(n: u64) -> u64 {
    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Clone, Copy, Debug)]
    struct Point {
        x: i64,
        y: i64,
    }

    let mut larger = 0;
    let mut dir = Direction::East;
    let mut dir_count = 1;
    let mut spiral: HashMap<(i64, i64), u64> = HashMap::new();

    spiral.insert((0, 0), 1);
    let mut last_coords = Point { x: 0, y: 0 };

    let mut d_count = dir_count;

    loop {
        let mut new_coords = last_coords;

        match dir {
            Direction::East => {
                new_coords.x += 1;
                d_count -= 1;

                if d_count == 0 {
                    dir = Direction::North;
                }
            }
            Direction::North => {
                new_coords.y += 1;
                d_count -= 1;

                if d_count == 0 {
                    dir = Direction::West;
                    dir_count += 1;
                }
            }
            Direction::South => {
                new_coords.y -= 1;
                d_count -= 1;

                if d_count == 0 {
                    dir = Direction::East;
                    dir_count += 1;
                }
            }
            Direction::West => {
                new_coords.x -= 1;
                d_count -= 1;

                if d_count == 0 {
                    dir = Direction::South;
                    //dir_count += 1;
                }
            }
        };

        if d_count == 0 {
            d_count = dir_count;
        }

        let mut sum = 0;

        let left = (new_coords.x - 1, new_coords.y);
        let up = (new_coords.x, new_coords.y + 1);
        let right = (new_coords.x + 1, new_coords.y);
        let down = (new_coords.x, new_coords.y - 1);
        let left_up = (new_coords.x - 1, new_coords.y + 1);
        let left_down = (new_coords.x - 1, new_coords.y - 1);
        let right_up = (new_coords.x + 1, new_coords.y + 1);
        let right_down = (new_coords.x + 1, new_coords.y - 1);

        if let Some(v) = spiral.get(&left) {
            sum += v;
        }

        if let Some(v) = spiral.get(&up) {
            sum += v;
        }

        if let Some(v) = spiral.get(&down) {
            sum += v;
        }

        if let Some(v) = spiral.get(&right) {
            sum += v;
        }

        if let Some(v) = spiral.get(&left_up) {
            sum += v;
        }

        if let Some(v) = spiral.get(&left_down) {
            sum += v;
        }

        if let Some(v) = spiral.get(&right_up) {
            sum += v;
        }

        if let Some(v) = spiral.get(&right_down) {
            sum += v;
        }

        if sum > n {
            println!("{}", sum);
            break;
        } else {
            spiral.insert((new_coords.x, new_coords.y), sum);
        }

        last_coords = new_coords;
    }

    larger
}
