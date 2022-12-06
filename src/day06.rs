use std::fs;
use std::collections::HashSet;

fn find_marker(slice: &[char], len: usize) -> usize {
    for (i, _) in slice.iter().enumerate() {
        if i + len >= slice.len() {
            break;
        }

        let chunk = &slice[i..i + len];
        let mut uniq = HashSet::new();
        let is_unique = chunk.iter().all(move |x| uniq.insert(x));

        if is_unique {
            return i + len;
        }
    }

    return 0;
}

pub fn day06_1() {
    let input = fs::read_to_string("./data/day06.txt").expect("Failed to read file");
    let chars = input.chars().collect::<Vec<char>>();
    let sliced = chars.as_slice();

    println!("First marker at: {}", find_marker(sliced, 4))
}

pub fn day06_2() {
    let input = fs::read_to_string("./data/day06.txt").expect("Failed to read file");
    let chars = input.chars().collect::<Vec<char>>();
    let sliced = chars.as_slice();

    println!("First marker at: {}", find_marker(sliced, 14))
}
