use std::fs;

fn get_sums() -> Vec<i32> {
    let calories_str = fs::read_to_string("./data/day1.txt").expect("Failed to read file");
    let calories_lines = calories_str.lines();

    let mut sums = Vec::<i32>::new();

    let mut tmp = 0;
    calories_lines.for_each(|f| {
        if f.len() != 0 {
            tmp += f.parse::<i32>().expect("Expected a number");
            return;
        }

        sums.push(tmp);
        tmp = 0;
    });

    return sums;
}

pub fn day1_1() {
    let mut sums = get_sums();

    sums.sort();
    println!("Biggest sum: {:?}", sums.last().expect("Expected a value"));
}

pub fn day1_2() {
    let mut sums = get_sums();

    sums.sort();
    sums.reverse();

    let top_three = sums.drain(..3);
    println!("Top 3 sum: {:?}", top_three.sum::<i32>());
}
