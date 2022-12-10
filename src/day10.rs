use std::fs;

// Could be off by a character
fn printer(cycle: i32, sprite_mid: i32) {
    let crt_index = cycle % 40 - 1 ;
    let normalized = sprite_mid % 40;
    let sprite_pos = vec![normalized - 1, normalized, normalized + 1];

    if sprite_pos.contains(&crt_index) {
        print!("#");
    } else {
        print!(".");
    }

    if cycle % 40 == 0 {
        print!("\n");
    }
}

fn process_input(input: String, should_printer: bool) -> i32 {
    let mut lines = input.lines().peekable();

    let mut cycle = 0;
    let mut register = 1;
    let mut signal_strength_sum = 0;
    let mut value_to_add: Option<i32> = None;

    loop {
        cycle += 1;

        // Peek to prevent printing an extra dot
        if should_printer && lines.peek().is_some() {
            printer(cycle, register);
        }

        //        println!("Current cycle: {}", cycle);
        //        println!("Value to add: {:?}", value_to_add);

        if cycle % 40 == 20 {
            let strength = cycle * register;
            signal_strength_sum += strength;
            //            println!("Cycle: {}, Register: {}, Strength: {}", cycle, register, strength);
        }

        if value_to_add.is_some() {
            register += value_to_add.expect("This should probably not happen");
            value_to_add = None;
            continue;
        }

        let line = match lines.next() {
            Some(x) => x,
            None => break,
        };

        if line.starts_with("noop") {
            continue;
        }

        if line.starts_with("addx") {
            let split = line.split_terminator(" ");
            value_to_add = Some(split.last().unwrap().parse().unwrap());
            continue;
        }
    }

    return signal_strength_sum;
}

pub fn day10_1() {
    let input = fs::read_to_string("./data/day10.txt").expect("Failed to read file");

    let sum = process_input(input, false);

    println!("Sum of signals: {}", sum);
}

pub fn day10_2() {
    let input = fs::read_to_string("./data/day10.txt").expect("Failed to read file");
    process_input(input, true);
}
