use aoc22_shared::*;
use std::{collections::HashMap, hash::Hash};

struct Directory {
    name: String,
    parent_ind: Option<usize>,
    children_dir_ind: HashMap<String, usize>,
    children_file_ind: HashMap<String, usize>,
}

struct File {
    name: String,
    size: u32
}

fn get_sizes(curr_dir_ind: usize, directories: &mut Vec<Directory>, files: &mut Vec<File>, dirs_size: &mut HashMap<usize, u32>) {
    let mut curr_dir = directories.get(curr_dir_ind).unwrap();
    if curr_dir.children_dir_ind.is_empty() {
        for file_ind in curr_dir.children_file_ind.values() {
            let curr_file_size = files.get(*file_ind).unwrap().size;

            *dirs_size.entry(curr_dir_ind).or_insert(0) += curr_file_size;
        }
    } else {
        let child_dirs_ind: Vec<usize> = curr_dir.children_dir_ind.values().map(|i| i.clone()).collect();

        for dir_ind in child_dirs_ind.iter() {
            get_sizes(*dir_ind, directories, files, dirs_size);
        }

        for dir_ind in child_dirs_ind.iter() {
            *dirs_size.entry(curr_dir_ind).or_insert(0) += *dirs_size.get_mut(dir_ind).unwrap();
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut directories: Vec<Directory> = Vec::new();
        directories.push(Directory { name: String::from("/"), parent_ind: None, children_dir_ind: HashMap::new(), children_file_ind: HashMap::new() });

        let mut files: Vec<File> = Vec::new();

        let mut curr_dir_ind: usize = 0;

        for line in lines.map_while(Result::ok) {
            let mut split_line = line.split(" ");

            if line.chars().next().unwrap() == '$' {
                match split_line.nth(1) {
                    Some("cd") => {
                        if let Some(dir) = split_line.next() {
                            if dir == "/" {
                                curr_dir_ind = 0;
                            } else if dir == ".." {
                                curr_dir_ind = directories[curr_dir_ind].parent_ind.unwrap();
                            } else {
                                if let Some(d) = directories[curr_dir_ind].children_dir_ind.get(dir) {
                                    curr_dir_ind = d.clone();
                                } else {
                                    panic!("Could not navigate to directory: {dir}");
                                }
                            }
                        } else {
                            panic!("No target directory specified for cd");
                        }
                    },
                    Some("ls") => {

                    },
                    Some(c) => panic!("Unexpected command: {c}"),
                    None => panic!("Command line doesn't have a command")
                }
            } else {
                let first_token = split_line.next().unwrap();
                if first_token == "dir" {
                    let dir_name = split_line.next().unwrap();
                    if !directories[curr_dir_ind].children_dir_ind.contains_key(dir_name) {
                        directories.push(Directory { name: String::from(dir_name), parent_ind: Some(curr_dir_ind), children_dir_ind: HashMap::new(), children_file_ind: HashMap::new() });
                        
                        let new_dir_ind = directories.len() - 1;
                        directories[curr_dir_ind].children_dir_ind.insert(String::from(dir_name), new_dir_ind);
                    }
                } else {
                    let file_size = first_token.parse::<u32>().unwrap();
                    let file_name = split_line.next().unwrap();
                    if !directories[curr_dir_ind].children_file_ind.contains_key(file_name) {
                        files.push(File{name: String::from(file_name), size: file_size});

                        let new_file_ind = files.len() - 1;
                        directories[curr_dir_ind].children_file_ind.insert(String::from(file_name), new_file_ind);
                    }
                }
            }
        }

        let mut dirs_size: HashMap<usize, u32> = HashMap::new();
        get_sizes(0, &mut directories, &mut files, &mut dirs_size);

        println!("{:?}", dirs_size)
    }
}
