use std::collections::HashMap;
use std::fs;

/// A, X -> Rock -> 0
///
/// B, Y -> Paper -> 1
///
/// C, Z -> Scissors -> 2
fn get_move_map<'a>() -> HashMap<&'a str, usize> {
    return HashMap::from([("A", 0), ("B", 1), ("C", 2), ("X", 0), ("Y", 1), ("Z", 2)]);
}

/// 0 -> Rock
///
/// 1 -> Paper
///
/// 2 -> Scissors
fn get_outcome_arr() -> Vec<Vec<usize>> {
    return Vec::from([
        Vec::from([3, 6, 0]),
        Vec::from([0, 3, 6]),
        Vec::from([6, 0, 3]),
    ]);
}

fn check_outcome(moves: Vec<&str>) -> usize {
    let move_map = get_move_map();
    let outcome_arr = get_outcome_arr();

    let opponent_mapped = move_map.get(moves[0]).expect("Unable to map opponent");
    let player_mapped = move_map.get(moves[1]).expect("Unable to map player");
    let outcome = outcome_arr
        .get(*opponent_mapped)
        .unwrap()
        .get(*player_mapped)
        .unwrap();

    //    println!("{} mapped to {}", moves[0], opponent_mapped);
    //    println!("{} mapped to {}", moves[1], player_mapped);
    //    println!("Outcome for {}, {} is {}", opponent_mapped, player_mapped, outcome);

    return outcome + player_mapped + 1;
}

fn fix_outcome(moves: Vec<&str>) -> usize {
    let move_map = get_move_map();
    let outcome_arr = get_outcome_arr();
    // X -> Lose (0p)
    // Y -> Draw (3p)
    // Z -> Win (6p)
    let req_outcomes = HashMap::<&str, usize>::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let opponent_mapped = move_map.get(moves[0]).expect("Unable to map opponent");
    let player_mapped = req_outcomes.get(moves[1]).expect("Unable to map player");
    let outcome = outcome_arr.get(*opponent_mapped).unwrap();

    let mut iter = outcome.iter();
    let outcome_index = iter.position(|&x| x == *player_mapped).expect("Expected to find a position");

    return player_mapped + outcome_index + 1;
}

pub fn day2_1() {
    let moves_str = fs::read_to_string("./data/day2.txt").expect("Failed to read file");
    let moves_lines = moves_str.lines();

    let mut total_outcome = 0;

    moves_lines.for_each(|line| {
        let separated = line.split(" ").collect::<Vec<&str>>();
        let outcome = check_outcome(separated);
        total_outcome += outcome;
//        println!("Outcome: {}", outcome);
    });

    println!("Total outcome: {}", total_outcome);
}

pub fn day2_2() {
    let moves_str = fs::read_to_string("./data/day2.txt").expect("Failed to read file");
    let moves_lines = moves_str.lines();

    let mut total_outcome = 0;

    moves_lines.for_each(|line| {
        let separated = line.split(" ").collect::<Vec<&str>>();
        let outcome = fix_outcome(separated);
        total_outcome += outcome;
//        println!("Outcome: {}", outcome);
    });

    println!("Total outcome: {}", total_outcome);
}
