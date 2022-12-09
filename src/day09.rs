use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::fs;

fn get_distance(pos_head: &(i32, i32), pos_tail: &(i32, i32)) -> i32 {
    return max(
        (pos_head.0 - pos_tail.0).abs(),
        (pos_head.1 - pos_tail.1).abs(),
    );
}

fn move_rope(
    pos_head: &mut (i32, i32),
    pos_tail: &mut (i32, i32),
    visited_tiles: &mut HashSet<(i32, i32)>,
    dir: &str,
    steps: i32,
) {
    // 0 -> y axis
    // 1 -> x axis

    for _ in 0..steps {
        // Handle right
        if dir == "R" {
            pos_head.1 += 1;
            if get_distance(pos_head, pos_tail) > 1 {
                pos_tail.0 = pos_head.0;
                pos_tail.1 = pos_head.1 - 1;
                visited_tiles.insert(*pos_tail);
            }
        }

        // Handle left
        if dir == "L" {
            pos_head.1 -= 1;
            if get_distance(pos_head, pos_tail) > 1 {
                pos_tail.0 = pos_head.0;
                pos_tail.1 = pos_head.1 + 1;
                visited_tiles.insert(*pos_tail);
            }
        }

        // Handle Up {
        if dir == "U" {
            pos_head.0 += 1;
            if get_distance(pos_head, pos_tail) > 1 {
                pos_tail.0 = pos_head.0 - 1;
                pos_tail.1 = pos_head.1;
                visited_tiles.insert(*pos_tail);
            }
        }

        // Handle Down {
        if dir == "D" {
            pos_head.0 -= 1;
            if get_distance(pos_head, pos_tail) > 1 {
                pos_tail.0 = pos_head.0 + 1;
                pos_tail.1 = pos_head.1;
                visited_tiles.insert(*pos_tail);
            }
        }
    }
}

pub fn day09_1() {
    let input = fs::read_to_string("./data/day09.txt").expect("Failed to read file");
    let lines = input.lines();

    let mut visited_tiles: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut pos_head: (i32, i32) = (0, 0);
    let mut pos_tail: (i32, i32) = (0, 0);

    lines.for_each(|f| {
        let mut split = f.split_terminator(" ");
        let dir = split.next().unwrap();
        let steps: i32 = split.next().unwrap().parse().unwrap();

        move_rope(&mut pos_head, &mut pos_tail, &mut visited_tiles, dir, steps);
    });

    println!("Visited count: {}", visited_tiles.len());
    //    println!("Pos head: {:?}, Pos tail: {:?}", pos_head, pos_tail);
}

pub fn day09_2() {
    let input = fs::read_to_string("./data/day09.txt").expect("Failed to read file");
    let lines = input.lines();

    let mut visited_tiles: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut pos_head = (0, 0);
    let mut tails: Vec<(i32, i32)> = vec![(0, 0); 9];

    lines.for_each(|f| {
        let mut split = f.split_terminator(" ");
        let direction = split.next().unwrap();
        let steps: i32 = split.next().unwrap().parse().unwrap();

        for _s in 0..steps {
            match direction {
                "R" => pos_head.1 += 1,
                "L" => pos_head.1 -= 1,
                "U" => pos_head.0 += 1,
                "D" => pos_head.0 -= 1,
                &_ => (),
            }

            for i in 0..tails.len() {
                let prev = if i == 0 {
                    pos_head
                } else {
                    tails.get(i - 1).unwrap().clone()
                };

                let mut current = tails.get_mut(i).unwrap();

                if get_distance(&prev, current) > 1 {
                    let diff = (prev.0 - current.0, prev.1 - current.1);

                    current.0 += max(-1, min(1, diff.0));
                    current.1 += max(-1, min(1, diff.1));
                }
            }

            visited_tiles.insert(*tails.last().unwrap());
            //            println!(f
            //                "Dir: {}, Step: {}, Head: {:?}, Tails: {:?}",
            //                direction, _s, pos_head, tails
            //            );
        }
    });

    println!("Visited: {}", visited_tiles.len());
}
