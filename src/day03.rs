use std::collections::HashMap;
use std::fs;

fn get_priority_from_char(c: char) -> u32 {
    let mut num = c as u32;
    num -= if num > 96 { 96 } else { 38 };
    return num;
}

pub fn day03_1() {
    let rucksacks_str = fs::read_to_string("./data/day03.txt").expect("Failed to read file");
    let rucksacks_lines = rucksacks_str.lines();

    let mut sum = 0;
    rucksacks_lines.for_each(|f| {
        let mut items = f.split_terminator("").skip(1).collect::<Vec<&str>>();

        let first_half = items.drain(..f.len() / 2).collect::<Vec<&str>>();
        let second_half = items.drain(..).collect::<Vec<&str>>();
        let mut occurrence_map = HashMap::<&str, i32>::new();
        //        println!("fh: {:?}", first_half);
        //        println!("sh: {:?}", second_half);

        first_half.iter().for_each(|f| {
            occurrence_map.insert(f, 0);
        });

        let mut recurring_char: Option<char> = None;
        for f in second_half {
            recurring_char = match occurrence_map.get(f) {
                Some(_) => f.chars().nth(0),
                None => continue,
            };

            break;
        }

        let priority = get_priority_from_char(recurring_char.expect("Expected a char"));
        sum += priority;
//        println!("{:?} ({})", priority, recurring_char.unwrap());
    });

    println!("Total sum: {}", sum);
}

pub fn day03_2() {
    let rucksacks_str = fs::read_to_string("./data/day03.txt").expect("Failed to read file");
    let rucksacks_lines = rucksacks_str.lines();

    let mut sum = 0;
    let mut counter = 0;
    let mut tmp_occurrence_map = HashMap::<&str, i32>::new();
    rucksacks_lines.for_each(|f| {
        let items = f.split_terminator("").skip(1).collect::<Vec<&str>>();

        // Required in case there are multiple same chars
        let mut tmp = HashMap::<&str, i32>::new();
        items.iter().for_each(|f| {
            tmp.insert(f, 1);
        });

        tmp.iter().for_each(|f| {
            let existing = tmp_occurrence_map.get(f.0);
            let mut new_value = 1;

            if existing.is_some() {
                new_value = existing.unwrap() + 1;
            }

            tmp_occurrence_map.insert(f.0, new_value);
        });
        counter += 1;

        // Reset every 3 lines
        if counter == 3 {
            let common = tmp_occurrence_map.iter().find(|i| *i.1 == 3).expect("Expected to find a common char");
//            println!("Common: {}", common.0);
            sum += get_priority_from_char(common.0.chars().nth(0).unwrap());
            tmp_occurrence_map.clear();
            counter = 0;
        }
    });

    println!("Total sum: {}", sum);
}
