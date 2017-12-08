use std::collections::HashMap;

#[derive(Debug)]
struct Tower {
    name: String,
    weight: usize,
    subtowers: Option<Vec<String>>,
}

fn main() {
    let input = include_str!("../input2.txt");
    println!("{}", day_7_part_1(input));
    day_7_part_2(input);
}

fn day_7_part_1(input: &str) -> String {
    let towers = parse_towers(input);

    let iter = towers.iter();
    let iter2 = iter.clone().filter(|t| t.subtowers.is_some());

    let mut iter = iter.filter(|t| t.subtowers.is_some()).peekable();

    let mut set_of_subtowers = Vec::new();

    for subtower_list in iter2.map(|t| t.subtowers.clone().unwrap()) {
        for subtower in subtower_list {
            set_of_subtowers.push(subtower);
        }
    }

    while set_of_subtowers.contains(&iter.peek().unwrap().name) {
        let _ = iter.next();
    }

    iter.next().unwrap().name.clone()
}

fn day_7_part_2(input: &str) -> usize {
    let mut weight = 0;
    let mut towers = HashMap::new();
    let mut towers_total_weight = HashMap::new();

    let base_tower = day_7_part_1(input);
    let vec_towers = parse_towers(input);

    for t in vec_towers {
        towers.insert(t.name, (t.weight, t.subtowers));
    }

    get_total_weights(&towers, &mut towers_total_weight, base_tower);

    println!("{:?}", towers_total_weight);

    weight
}

fn find_divergent_weight(
    towers: &HashMap<String, (usize, Option<Vec<String>>)>,
    towers_total_weight: &HashMap<String, usize>,
    start: String,
) -> Option<usize> {
    let mut div_w = 0;

    let children = towers.get(&start).unwrap();

    if 
}

fn get_total_weights(
    towers: &HashMap<String, (usize, Option<Vec<String>>)>,
    towers_total_weight: &mut HashMap<String, usize>,
    start: String,
) -> usize {
    let mut size = 0;

    let subtowers = towers.get(&start).unwrap();

    if let None = subtowers.1 {
        towers_total_weight.insert(start, subtowers.0);
        return subtowers.0;
    } else {
        size += subtowers.0;
        let subtowers = subtowers.1.as_ref().unwrap();
        for subtower in subtowers {
            size += get_total_weights(towers, towers_total_weight, subtower.clone());
        }
    }

    towers_total_weight.insert(start, size);
    size
}

fn parse_towers(input: &str) -> Vec<Tower> {
    let mut towers = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let name = parts.next().unwrap().to_string();
        let weight = parts
            .next()
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .parse()
            .unwrap();

        let mut subtowers = Vec::new();

        while let Some(s) = parts.next() {
            if s != "->" {
                subtowers.push(s.trim_right_matches(',').to_string());
            }
        }

        towers.push(Tower {
            name: name,
            weight: weight,
            subtowers: if subtowers.len() >= 1 {
                Some(subtowers)
            } else {
                None
            },
        })
    }

    towers
}
