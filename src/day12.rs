use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    dist: usize,
    elevation: usize,
}

/// Convert input into a 2d vector of usize values
fn map_input(data: String) -> Vec<Vec<usize>> {
    let lines = data.lines();

    return lines
        .map(|f| f.chars().map(|c| c as usize).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
}

/// Check if we can move depending on source and dest elevation
fn can_move(from: usize, to: usize) -> bool {
    let to_adjusted = if to == 69 { 122 } else { to };

    // We can only go down or 1 up
    if to_adjusted <= from || from + 1 == to_adjusted {
        return true;
    }

    return false;
}

/// Find all sources of specified char
fn find_sources(c: char, map: &Vec<Vec<usize>>) -> Vec<Node> {
    let height = map.len();
    let width = map.first().unwrap().len();

    let mut sources: Vec<Node> = vec![];
    // Change elevation of S
    let elevation = if c == 'S' { 97 } else { c as usize };

    // Find specified character
    for i in 0..height {
        for j in 0..width {
            if map[i][j] == c as usize {
                sources.push(Node {
                    x: j,
                    y: i,
                    dist: 0,
                    elevation: elevation,
                });
            }
        }
    }

    return sources;
}

fn min_distance(source: Node, map: &Vec<Vec<usize>>) -> usize {
    let height = map.len();
    let width = map.first().unwrap().len();
    let mut visited = vec![vec![false; width]; height];

    let mut queue: VecDeque<Node> = VecDeque::new();
    visited[source.y][source.x] = true;
    queue.push_back(source);

    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();

        // Check if destination is found
        if map[current_node.y][current_node.x] == 69 {
            return current_node.dist;
        }

        // Try moving up
        if current_node.y as i32 - 1 >= 0
            && visited[current_node.y - 1][current_node.x] == false
            && can_move(current_node.elevation, map[current_node.y - 1][current_node.x])
        {
            queue.push_back(Node {
                y: current_node.y - 1,
                x: current_node.x,
                dist: current_node.dist + 1,
                elevation: map[current_node.y - 1][current_node.x],
            });
            visited[current_node.y - 1][current_node.x] = true;
        }

        // Try moving left
        if current_node.x as i32 - 1 >= 0
           && visited[current_node.y][current_node.x - 1] == false
           && can_move(current_node.elevation, map[current_node.y][current_node.x - 1])
        {
            queue.push_back(Node {
                y: current_node.y,
                x: current_node.x - 1,
                dist: current_node.dist + 1,
                elevation: map[current_node.y][current_node.x - 1],
            });
            visited[current_node.y][current_node.x - 1] = true;
        }

        // Try moving down
        if current_node.y + 1 < height
            && visited[current_node.y + 1][current_node.x] == false
            && can_move(current_node.elevation, map[current_node.y + 1][current_node.x])
        {
            queue.push_back(Node {
                y: current_node.y + 1,
                x: current_node.x,
                dist: current_node.dist + 1,
                elevation: map[current_node.y + 1][current_node.x],
            });
            visited[current_node.y + 1][current_node.x] = true;
        }

        // Try moving right
        if current_node.x + 1 < width
            && visited[current_node.y][current_node.x + 1] == false
            && can_move(current_node.elevation, map[current_node.y][current_node.x + 1])
        {
            queue.push_back(Node {
                y: current_node.y,
                x: current_node.x + 1,
                dist: current_node.dist + 1,
                elevation: map[current_node.y][current_node.x + 1],
            });
            visited[current_node.y][current_node.x + 1] = true;
        }
    }

    return 0;
}

pub fn day12_1() {
    let data = fs::read_to_string("./data/day12.txt").expect("Failed to read file");

    let map = map_input(data);
    let sources = find_sources('S', &map);

    // Part 1 only has 1 source
    let source = sources.first().unwrap();
    let min = min_distance(*source, &map);
    println!("Min: {}", min);
}

pub fn day12_2() {
    let data = fs::read_to_string("./data/day12.txt").expect("Failed to read file");

    let map = map_input(data);
    let sources = find_sources('a', &map);

    let mut possible_distances: Vec<usize> = vec![];

    for source in sources {
        let dist = min_distance(source, &map);

        if dist == 0 {
            continue;
        }

        possible_distances.push(dist);
    }

    possible_distances.sort();
    println!("Min: {}", possible_distances.first().unwrap());
}
