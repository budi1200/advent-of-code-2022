use regex::Regex;
use std::fs;

fn parse_cargo(data: &str) -> Vec<Vec<&str>> {
    let mut cargo: Vec<Vec<&str>> = Vec::new();

    data.lines().rev().skip(1).for_each(|f| {
        let mut iter = f.split_terminator("").into_iter();
        let mut index = 0;

        while iter.next().is_some() {
            // Get group of 3 chars (either three spaces or [X])
            let group = iter.by_ref().take(3).collect::<Vec<&str>>();
            // We are interested only in the middle character
            let value = group.get(1).unwrap();

            // Add empty vector if col is empty
            if cargo.get(index).is_none() {
                cargo.push(Vec::new());
            }

            // Push box to current col
            if *value != " " {
                cargo[index].push(value);
            }

            index += 1;
        }
    });

    //    println!("{:?}", cargo);
    return cargo;
}

pub fn day05_1() {
    let data_str = fs::read_to_string("./data/day05.txt").expect("Failed to read file");
    let mut sections = data_str.split_terminator("\n\n");
    let re = Regex::new(r"([0-9]+)").unwrap();

    let mut cargo = parse_cargo(sections.next().unwrap()).clone();
    let moves = sections.next().unwrap();

    moves.lines().for_each(|f| {
        //        println!("{:?}", f);
        let parsed_moves = re
            .find_iter(f)
            .map(|cap| cap.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let source_col = cargo.get_mut(parsed_moves[1] - 1).unwrap();
        let cache = source_col.split_off(source_col.len() - parsed_moves[0]);
        let mut reversed = cache.into_iter().rev().collect::<Vec<&str>>();

        cargo.get_mut(parsed_moves[2] - 1).unwrap().append(&mut reversed);
    });

    let mut output = String::new();

    cargo.iter().for_each(|f| {
        output += f.last().unwrap();
    });

    //    println!("{:?}", cargo);
    println!("{:?}", output);
}

pub fn day05_2() {
    let data_str = fs::read_to_string("./data/day05.txt").expect("Failed to read file");
    let mut sections = data_str.split_terminator("\n\n");
    let re = Regex::new(r"([0-9]+)").unwrap();

    let mut cargo = parse_cargo(sections.next().unwrap()).clone();
    let moves = sections.next().unwrap();

    moves.lines().for_each(|f| {
        //        println!("{:?}", f);
        let parsed_moves = re
        .find_iter(f)
        .map(|cap| cap.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

        let source_col = cargo.get_mut(parsed_moves[1] - 1).unwrap();
        let mut cache = source_col.split_off(source_col.len() - parsed_moves[0]);

        cargo.get_mut(parsed_moves[2] - 1).unwrap().append(&mut cache);
    });

    let mut output = String::new();

    cargo.iter().for_each(|f| {
        output += f.last().unwrap();
    });

    //    println!("{:?}", cargo);
    println!("{:?}", output);
}
