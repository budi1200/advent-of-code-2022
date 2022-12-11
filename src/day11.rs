use std::fs;

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i64>,
    operation: char,
    operation_value: String,
    divisible_by: i64,
    divisible_true: i32,
    divisible_false: i32,
    inspections: i64,
}

pub fn parse_monkeys(input: String) -> Vec<Monkey> {
    let monkeys = input.split_terminator("\n\n").collect::<Vec<&str>>();

    let mut parsed_monkeys: Vec<Monkey> = vec![];

    monkeys.iter().for_each(|f| {
        // To lines & skip first line
        let mut lines = f.lines().skip(1);

        let mut items = lines
            .next()
            .unwrap()
            .replace("Starting items: ", "")
            .split_terminator(", ")
            .collect::<Vec<&str>>()
            .iter_mut()
            .map(|f| f.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        items.reverse();

        // I have no idea what is happening here
        // Something something "borrowed value does not live long enough"
        // I should probably properly learn rust
        let a = lines.next().unwrap();
        let b = a.replace("Operation: new = old ", "");
        let mut operation_tmp = b.trim().split_terminator(" ");

        let operation = operation_tmp.next().unwrap().chars().last().unwrap();
        let operation_value = operation_tmp.next().unwrap().to_owned();

        let divisible_by = lines
            .next()
            .unwrap()
            .replace("Test: divisible by", "")
            .trim()
            .parse::<i64>()
            .unwrap();

        let divisible_true = lines
            .next()
            .unwrap()
            .replace("If true: throw to monkey", "")
            .trim()
            .parse::<i32>()
            .unwrap();

        let divisible_false = lines
            .next()
            .unwrap()
            .replace("If false: throw to monkey", "")
            .trim()
            .parse::<i32>()
            .unwrap();

        let tmp = Monkey {
            items,
            operation,
            operation_value,
            divisible_by,
            divisible_true,
            divisible_false,
            inspections: 0,
        };

        parsed_monkeys.push(tmp);
    });

    //    println!("Parsed monkeys: {:?}", parsed_monkeys);
    return parsed_monkeys;
}

fn inspect_item(monkey: &mut Monkey, worry_manager: i64) -> i64 {
    let mut item = monkey.items.pop().unwrap();
    monkey.inspections += 1;
    //    println!("Inspecting item: {}", item);

    // Handle operation
    let value = match monkey.operation_value.as_str() {
        "old" => item,
        x => x.parse::<i64>().unwrap(),
    };

    match monkey.operation {
        '+' => item += value,
        '-' => item -= value,
        '*' => item *= value,
        _ => (),
    }

    //    println!("Oh no! Worry is now: {}", item);
    if worry_manager == 3 {
        item /= worry_manager;
    } else {
        item %= worry_manager;
    }
    //    println!("Monkey bored: {}", item);

    return item;
}

fn handle_monkey(monkeys: &mut Vec<Monkey>, ind: usize, worry_manager: i64) {
    loop {
        if monkeys.get(ind).unwrap().items.len() == 0 {
            break;
        }

        let item = inspect_item(monkeys.get_mut(ind).unwrap(), worry_manager);
        let readonly_monkey = monkeys.get(ind).unwrap().clone();

        // Something something double borrow
        let divisible_by = readonly_monkey.divisible_by;
        let divisible_true = readonly_monkey.divisible_true as usize;
        let divisible_false = readonly_monkey.divisible_false as usize;

        match item % divisible_by == 0 {
            true => monkeys
                .get_mut(divisible_true)
                .unwrap()
                .items
                .insert(0, item),
            false => monkeys
                .get_mut(divisible_false)
                .unwrap()
                .items
                .insert(0, item),
        }
    }
}

pub fn day11_1() {
    let input = fs::read_to_string("./data/day11.txt").expect("Failed to read file");

    let mut monkeys = parse_monkeys(input);

    for _ in 0..20 {
        for ind in 0..monkeys.len() {
            handle_monkey(&mut monkeys, ind, 3);
        }
    }

    let mut inspections = monkeys.iter().map(|f| f.inspections).collect::<Vec<i64>>();
    inspections.sort();
    inspections.reverse();

    let monkey_business = inspections.get(0).unwrap() * inspections.get(1).unwrap();

    //    println!("Inspections: {:?}", inspections);
    //    println!("Monkey data: {:#?}", monkeys);
    println!("Monkey business level: {:#?}", monkey_business);
}

pub fn day11_2() {
    let input = fs::read_to_string("./data/day11.txt").expect("Failed to read file");

    let mut monkeys = parse_monkeys(input);
    let ugh = monkeys
        .iter()
        .map(|f| f.divisible_by)
        .reduce(|acc, monkey| acc * monkey)
        .unwrap();

    for _ in 0..10000 {
        for ind in 0..monkeys.len() {
            handle_monkey(&mut monkeys, ind, ugh);
        }
    }

    let mut inspections = monkeys.iter().map(|f| f.inspections).collect::<Vec<i64>>();

    inspections.sort();
    inspections.reverse();

    let monkey_business = inspections.get(0).unwrap() * inspections.get(1).unwrap();

    //        println!("Inspections: {:?}", inspections);
    //        println!("Monkey data: {:#?}", monkeys);
    println!("Monkey business level: {:#?}", monkey_business);
}
