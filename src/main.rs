mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use std::collections::HashMap;
use std::env;

fn get_available_days<'a>() -> HashMap<&'a str, fn()> {
    return HashMap::from([
        ("1.1", day01::day01_1 as fn()),
        ("1.2", day01::day01_2 as fn()),
        ("2.1", day02::day02_1 as fn()),
        ("2.2", day02::day02_2 as fn()),
        ("3.1", day03::day03_1 as fn()),
        ("3.2", day03::day03_2 as fn()),
        ("4.1", day04::day04_1 as fn()),
        ("4.2", day04::day04_2 as fn()),
        ("5.1", day05::day05_1 as fn()),
        ("5.2", day05::day05_2 as fn()),
        ("6.1", day06::day06_1 as fn()),
        ("6.2", day06::day06_2 as fn()),
        ("7.1", day07::day07_1 as fn()),
        ("7.2", day07::day07_2 as fn()),
        ("8.1", day08::day08_1 as fn()),
        ("8.2", day08::day08_2 as fn()),
        ("9.1", day09::day09_1 as fn()),
        ("9.2", day09::day09_2 as fn()),
    ]);
}

// Usage: cargo run <day>.<part>
// Example: cargo run 1.2
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return println!("Invalid arguments provided \nUsage: cargo run <day>.<part>");
    }

    let day = &args[1];

    println!("Chosen Day {}", day);

    let completed_days = get_available_days();

    let day_to_run = completed_days.get_key_value(&day as &str);

    if day_to_run == None {
        return println!("Day not found!");
    }

    // Run function for selected day
    day_to_run.unwrap().1();
}
