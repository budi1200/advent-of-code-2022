use std::collections::HashMap;
use std::fs;

fn format_path(path: &Vec<String>) -> String {
    return "/".to_string() + &path.join("/");
}

fn parse_file_system(data: String) -> HashMap<String, Vec<String>> {
    let mut lines = data.lines().peekable();
    let mut current_path: Vec<String> = vec![];
    let mut file_system = HashMap::<String, Vec<String>>::new();

    loop {
        let line = match lines.next() {
            Some(x) => x,
            None => break, // Exit after last line
        };

        // Handle going back with cd..
        if line.starts_with("$ cd ..") {
            current_path.pop();
            continue;
        }

        // Handle directory browse
        if line.starts_with("$ cd") {
            if line != "$ cd /" {
                current_path.push(line.replace("$ cd ", ""));
            }
        }

        if line.starts_with("$ ls") {
            //            println!("Contents of '{}'", format_path(&current_path));
            let mut dir_content: Vec<String> = vec![];

            while let Some(l) = lines.next_if(|&f| !f.starts_with("$")) {
                if l.starts_with("dir") {
                    // Avoid double //
                    let separator = if current_path.len() == 0 { "" } else { "/" };
                    let dir_path = format_path(&current_path) + &l.replace("dir ", separator);
                    dir_content.push(dir_path);
                } else {
                    // We don't care about file names
                    let mut split = l.split_terminator(" ");
                    dir_content.push(split.next().unwrap().to_string());
                }
            }

            file_system.insert(format_path(&current_path), dir_content);
        }
    }

    return file_system;
}

fn get_dir_size(
        file_system: &HashMap<String, Vec<String>>,
        dir_sizes: &mut HashMap<String, usize>,
        path: &str,
        ) -> usize {

    let dir = file_system.get(path).unwrap();
    let mut size = 0;

    dir.iter().for_each(|f| {
        if f.starts_with("/") {
            size += get_dir_size(file_system, dir_sizes, f);
            return;
        }

        size += f.parse::<usize>().expect("Error parsing size");
    });

    // This could probably be done better
    dir_sizes.insert(path.to_string(), size);

    return size;
}

pub fn day07_1() {
    let input = fs::read_to_string("./data/day07.txt").expect("Failed to read file");

    let file_system = parse_file_system(input);
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    get_dir_size(&file_system, &mut dir_sizes, "/"); // Populate dir_sizes

    //    println!("File system: {:?}", file_system);
    //    println!("Sizes: {:?}", dir_sizes);

    let filtered = dir_sizes.into_values().filter(|f| f <= &100000).collect::<Vec<usize>>();
    let sum: usize = filtered.iter().sum();

    println!("Sum under 10k: {}", sum);
}

pub fn day07_2() {
    let input = fs::read_to_string("./data/day07.txt").expect("Failed to read file");

    let file_system = parse_file_system(input);
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    get_dir_size(&file_system, &mut dir_sizes, "/"); // Populate dir_sizes

    //    println!("File system: {:?}", file_system);
    //    println!("Sizes: {:?}", dir_sizes);

    let total_space = 70000000;
    let req_space = 30000000;

    let mut sizes: Vec<usize> = dir_sizes.into_values().collect();
    sizes.sort_unstable();

    let used_space = sizes.pop().unwrap();
    let needed_space = used_space + req_space - total_space;

    //    println!("Needed space: {}", needed_space);

    let mut filtered = sizes.iter().filter(|f| f >= &&needed_space);
    let deleted_dir_size = filtered.next().unwrap();

    println!("Deleting dir: {}", deleted_dir_size);
}
