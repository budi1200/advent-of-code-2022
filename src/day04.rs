use std::fs;

pub fn day04_1() {
    let data_str = fs::read_to_string("./data/day04.txt").expect("Failed to read file");
    let data_lines = data_str.lines();

    let mut overlaps = 0;
    data_lines.for_each(|f| {
        let mut split = f.split_terminator(",");
        let mut part1 = split.next().unwrap().split_terminator("-");
        let mut part2 = split.next().unwrap().split_terminator("-");

        let a: (i32, i32) = (part1.next().unwrap().parse().unwrap(), part1.next().unwrap().parse().unwrap());
        let b: (i32, i32) = (part2.next().unwrap().parse().unwrap(), part2.next().unwrap().parse().unwrap());

        if (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1) {
            overlaps += 1;
            //            println!("Match, {:?}", f);
        }
    });

    println!("Overlaps: {}", overlaps);
}

pub fn day04_2() {
    let data_str = fs::read_to_string("./data/day04.txt").expect("Failed to read file");
    let data_lines = data_str.lines();

    let mut overlaps = 0;
    data_lines.for_each(|f| {
        let mut split = f.split_terminator(",");
        let mut part1 = split.next().unwrap().split_terminator("-");
        let mut part2 = split.next().unwrap().split_terminator("-");

        let a: (i32, i32) = (part1.next().unwrap().parse().unwrap(), part1.next().unwrap().parse().unwrap());
        let b: (i32, i32) = (part2.next().unwrap().parse().unwrap(), part2.next().unwrap().parse().unwrap());

        if (a.0 <= b.1 && a.1 >= b.0) || (b.0 <= a.1 && b.1 >= a.0) {
            overlaps += 1;
//            println!("Match, {:?}", f);
        }
    });

    println!("Overlaps: {}", overlaps);
}
