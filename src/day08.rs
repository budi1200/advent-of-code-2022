use std::collections::HashMap;
use std::fs;

fn parse_input(data: String) -> Vec<Vec<usize>> {
    let lines = data.lines();
    let mut parsed: Vec<Vec<usize>> = vec![];

    lines.for_each(|f| {
        let split = f
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        parsed.push(split);
    });

    return parsed;
}

// Could be optimized with early returns
fn check_tree(
    tree_layout: &Vec<Vec<usize>>,
    tree: (usize, usize),
) -> (bool, usize) {
    let width = tree_layout.first().unwrap().len();
    let height = tree_layout.len();

    let mut visible_sides = HashMap::from([("t", true), ("l", true), ("b", true), ("r", true)]);
    let mut score = HashMap::from([("t", 0), ("l", 0), ("b", 0), ("r", 0)]);

    // 0 -> height
    // 1 -> width
    let tree_height = tree_layout.get(tree.0).unwrap().get(tree.1).unwrap();
//    println!("Tree height at {:?} is {}", tree, tree_height);

    // Check top
    for t in (0..tree.0).rev() {
        if tree_layout[t][tree.1] >= *tree_height {
            visible_sides.insert("t", false);
            score.insert("t", tree.0 - t);
            break;
        }

        score.insert("t", tree.0 - t);
    }

    // Check left
    for l in (0..tree.1).rev() {
        if tree_layout[tree.0][l] >= *tree_height {
            visible_sides.insert("l", false);
            score.insert("l", tree.1 - l);
            break;
        }

        score.insert("l", tree.1 - l);
    }

    // Check bottom
    for b in tree.0 + 1..height {
        if tree_layout[b][tree.1] >= *tree_height {
            visible_sides.insert("b", false);
            score.insert("b", b - tree.0);
            break;
        }

        score.insert("b", b - tree.0);
    }

    // Check right
    for r in tree.1 + 1..width {
        if tree_layout[tree.0][r] >= *tree_height {
            visible_sides.insert("r", false);
            score.insert("r", r - tree.1);
            break;
        }

        score.insert("r", r - tree.1);
    }

//    println!("Sides status: {:?}", visible_sides);
    let mut total_score = 1;

    score.into_values().for_each(|f| {
        total_score *= f;
    });

    return (visible_sides.into_values().any(|f| f == true), total_score);
}

pub fn day08_1() {
    let input = fs::read_to_string("./data/day08.txt").expect("Failed to read file");

    let tree_layout = parse_input(input);
    let layout_width = tree_layout.first().unwrap().len();
    let layout_height = tree_layout.len();

    let mut visible_trees = layout_width * 2 + layout_height * 2 - 4;

    for h in 1..layout_height - 1 {
        for w in 1..layout_width - 1 {
            if check_tree(&tree_layout, (h, w)).0 {
                visible_trees += 1;
            }
        }
    }

    println!("Visible trees: {}", visible_trees);
}

pub fn day08_2() {
    let input = fs::read_to_string("./data/day08.txt").expect("Failed to read file");

    let tree_layout = parse_input(input);
    let layout_width = tree_layout.first().unwrap().len();
    let layout_height = tree_layout.len();

    let mut scores: Vec<usize> = vec![];

    for h in 1..layout_height - 1 {
        for w in 1..layout_width - 1 {
            scores.push(check_tree(&tree_layout, (h, w)).1);
        }
    }

    scores.sort();
    scores.reverse();
    println!("Best score: {}", scores.first().unwrap());
}
