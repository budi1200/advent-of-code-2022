use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Entry {
    Integer(i32),
    Array(Vec<Entry>),
}

#[derive(PartialEq)]
enum State {
    True,
    False,
    Neutral,
}

/// Accepts left and right side array, returns if pair is in order
fn handle_pair_compare(left: &Vec<Entry>, right: &Vec<Entry>) -> State {
    let mut counter = 0;

    loop {
        if counter >= left.len() || counter >= right.len() {
            if left.len() == right.len() {
                break;
            }

            if left.len() < right.len() {
                return State::True;
            }

            return State::False;
        }

        let combo = (left.get(counter).unwrap(), right.get(counter).unwrap());

        //        println!("Compare {:?}", combo);
        match combo {
            (Entry::Integer(l), Entry::Integer(r)) => {
                if l > r {
                    //                    println!("Right side is smaller, so inputs are not in the right order");
                    return State::False;
                }

                if r > l {
                    //                    println!("Left side is smaller, so inputs are in the right order");
                    return State::True;
                }
            }
            (Entry::Array(l), Entry::Array(r)) => {
                let res = handle_pair_compare(l, r);

                if res != State::Neutral {
                    return res;
                }
            }
            (Entry::Integer(l), Entry::Array(r)) => {
                let mut tmp = Vec::<Entry>::new();
                tmp.push(Entry::Integer(*l));

                let res = handle_pair_compare(&tmp, r);
                if res != State::Neutral {
                    return res;
                }
            }
            (Entry::Array(l), Entry::Integer(r)) => {
                let mut tmp = Vec::<Entry>::new();
                tmp.push(Entry::Integer(*r));

                let res = handle_pair_compare(l, &tmp);
                if res != State::Neutral {
                    return res;
                }
            }
        }

        counter += 1;
    }

    //    println!("Neutral");
    return State::Neutral;
}

fn find_divider(vec: &Vec<Entry>, num: i32) -> bool {
    let first = vec.first();

    if first.is_none() {
        return false;
    }

    match first.unwrap() {
        Entry::Integer(_) => return false,
        Entry::Array(arr) => {
            let first2 = arr.first();

            if first2.is_none() {
                return false;
            }

            match first2.unwrap() {
                Entry::Integer(x) => return x == &num,
                Entry::Array(_) => return false,
            };
        }
    }
}

pub fn day13_1() {
    let data = fs::read_to_string("./data/day13.txt").expect("Failed to read file");
    let pairs = data.split_terminator("\n\n");

    let mut correct: Vec<usize> = vec![];

    pairs.enumerate().for_each(|(index, f)| {
        let mut tmp = f.split_terminator("\n");
        let part_1: Vec<Entry> = serde_json::from_str(tmp.next().unwrap()).unwrap();
        let part_2: Vec<Entry> = serde_json::from_str(tmp.next().unwrap()).unwrap();

        //        println!("Pair: {:?}, {:?}", part_1, part_2);
        let processed = handle_pair_compare(&part_1, &part_2);

        if processed == State::True {
            correct.push(index + 1);
        }

        //        println!("State of correct after pair {}: {:?}", index + 1, correct);
    });

    println!("Sum of indices: {}", correct.iter().sum::<usize>());
}

pub fn day13_2() {
    let mut data = fs::read_to_string("./data/day13.txt").expect("Failed to read file");

    // Add divider
    data.push_str("\n[[2]]");
    data.push_str("\n[[6]]");

    let mut tmp: Vec<Vec<Entry>> = data
        .lines()
        .filter(|f| f != &"")
        .map(|f| serde_json::from_str(f).unwrap())
        .collect();

    tmp.sort_by(|l, r| {
        let res = handle_pair_compare(l, r);

        if res == State::True {
            return Ordering::Less;
        }

        if res == State::False {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    });

    let first_key = tmp.iter().position(|f| find_divider(f, 2)).unwrap() + 1;
    let second_key = tmp.iter().position(|f| find_divider(f, 6)).unwrap() + 1;

    println!("Decoder key: {:?}", first_key * second_key);
}
