use std::cmp::max;
use std::cmp::min;
use std::fs;

// Plenty of optimization could still be done but it's fine

//fn printer(blocked_pos: Vec<(usize, usize)>, sand: Vec<(usize, usize)>) {
//    let mut lcd = vec![vec!["_"; 550]; 12];
//
//    blocked_pos.iter().for_each(|f| lcd[f.1][f.0] = "#");
//    sand.iter().for_each(|f| lcd[f.1][f.0] = "o");
//
//    lcd[11] = vec!["#"; 550];
//
//    lcd.iter_mut().for_each(|f| {
//        f.drain(..470);
//        println!("{:?}", f.join(""));
//    });
//}

fn parse_input(data: String) -> (Vec<(usize, usize)>, usize) {
    let lines = data.lines();
    let mut blocked_positions: Vec<(usize, usize)> = vec![];
    let mut cave_depth = 0;

    lines.for_each(|f| {
        let mut coords = f.split_terminator(" -> ").peekable();

        while coords.peek().is_some() {
            let mut _s = coords.next().unwrap().split_terminator(",");
            let start: (usize, usize) = (
                _s.next().unwrap().parse().unwrap(),
                _s.next().unwrap().parse().unwrap(),
            );

            let _tmp = coords.peek();
            // Handle last element
            if _tmp.is_none() {
                break;
            }

            let mut _e = _tmp.unwrap().split_terminator(",");
            let end: (usize, usize) = (
                _e.next().unwrap().parse().unwrap(),
                _e.next().unwrap().parse().unwrap(),
            );

            //            println!("{:?}, {:?}", start, end);
            blocked_positions.push(start);
            blocked_positions.push(end);

            // Keep track of cave depth
            if end.1 > cave_depth {
                cave_depth = end.1;
            }

            // If x axis is the same
            if start.0 == end.0 {
                for i in min(start.1, end.1) + 1..max(start.1, end.1) {
                    //                    println!("Adding: {}, {}", start.0, i);
                    blocked_positions.push((start.0, i));
                }
            }

            // If y axis is the same
            if start.1 == end.1 {
                for i in min(start.0, end.0) + 1..max(start.0, end.0) {
                    //                    println!("Adding: {}, {}", i, start.1);
                    blocked_positions.push((i, start.1));
                }
            }
        }
    });

    return (blocked_positions, cave_depth);
}

pub fn day14_1() {
    let data = fs::read_to_string("./data/day14.txt").expect("Failed to read file");

    let (mut blocked_positions, cave_depth) = parse_input(data);

    // Sort so we can do binary search
    blocked_positions.sort_unstable();

    let mut used_sand = 0;
    loop {
        // Drop the sand
        let mut sand_pos: (usize, usize) = (500, 0);

        loop {
            sand_pos.1 += 1;
            //            println!("sand_pos: {:?}", sand_pos);

            // Check if spot is occupied
            let mut is_taken = blocked_positions.binary_search(&sand_pos);

            // Check bottom
            if is_taken.is_ok() {
                sand_pos.0 -= 1;

                is_taken = blocked_positions.binary_search(&sand_pos);

                // Check diagonal left
                if is_taken.is_ok() {
                    sand_pos.0 += 2;

                    is_taken = blocked_positions.binary_search(&sand_pos);

                    // Check diagonal right
                    if is_taken.is_ok() {
                        sand_pos.0 -= 1;
                        sand_pos.1 -= 1;

                        used_sand += 1;
                        let ins = blocked_positions.partition_point(|x| x < &sand_pos);
                        blocked_positions.insert(ins, sand_pos);
                        break;
                    }
                }
            }

            if sand_pos.1 > cave_depth {
                break;
            }
        }

        if sand_pos.1 > cave_depth {
            break;
        }
    }

    //    println!("Depth: {}, Blocked: {:?}", cave_depth, blocked_positions);
    println!("Used Sand: {}", used_sand);
}

pub fn day14_2() {
    let data = fs::read_to_string("./data/day14.txt").expect("Failed to read file");

    let (mut blocked_positions, cave_depth) = parse_input(data);

    // Sort so we can do binary search
    blocked_positions.sort_unstable();

    let mut used_sand = 0;
    loop {
        // Drop the sand
        let mut sand_pos: (usize, usize) = (500, 0);

        loop {
            sand_pos.1 += 1;
            //            println!("sand_pos: {:?}", sand_pos);

            // Check if spot is occupied
            let mut is_taken = blocked_positions.binary_search(&sand_pos);

            // Check bottom
            if is_taken.is_ok() {
                sand_pos.0 -= 1;

                is_taken = blocked_positions.binary_search(&sand_pos);

                // Check diagonal left
                if is_taken.is_ok() {
                    sand_pos.0 += 2;

                    is_taken = blocked_positions.binary_search(&sand_pos);

                    // Check diagonal right
                    if is_taken.is_ok() {
                        sand_pos.0 -= 1;
                        sand_pos.1 -= 1;

                        // Exit when we reach top
                        if sand_pos == (500, 0) {
                            break;
                        }

                        used_sand += 1;
                        let ins = blocked_positions.partition_point(|x| x < &sand_pos);
                        blocked_positions.insert(ins, sand_pos);
                        break;
                    }
                }
            }

            // "Virtual" bottom
            if sand_pos.1 > cave_depth {
                used_sand += 1;
                let ins = blocked_positions.partition_point(|x| x < &sand_pos);
                blocked_positions.insert(ins, sand_pos);
                break;
            }
        }

        // Exit when we reach the top and add last piece
        if sand_pos.1 == 0 {
            used_sand += 1;
            let ins = blocked_positions.partition_point(|x| x < &sand_pos);
            blocked_positions.insert(ins, sand_pos);
            break;
        }
    }

    //    println!("Depth: {}, Blocked: {:?}", cave_depth, blocked_positions);
    println!("Used Sand: {}", used_sand);
}
