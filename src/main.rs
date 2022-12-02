mod day1;
mod day2;

use std::collections::HashMap;
use std::env;

fn get_available_days<'a>() -> HashMap<&'a str, fn()> {
    return HashMap::from([
        ("1.1", day1::day1_1 as fn()),
        ("1.2", day1::day1_2 as fn()),
        ("2.1", day2::day2_1 as fn()),
        ("2.2", day2::day2_2 as fn())
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
