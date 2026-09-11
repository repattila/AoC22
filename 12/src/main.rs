use std::process::exit;
use std::vec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use aoc22_shared::*;

fn main() {
    let mut field: Vec<Vec<u32>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut a_fields: Vec<(usize, usize)> = Vec::new();

    if let Ok(lines) = read_lines("res/input.txt") {
        for (r, line) in lines.map_while(Result::ok).enumerate() {
            let mut row: Vec<u32> = Vec::new();

            for (c, val) in line.chars().enumerate() {
                if val == 'E' {
                    row.push(26);

                    end = (r, c);
                } else if val == 'S' {
                    row.push(99);

                    start = (r, c);
                } else if val == 'a' {
                    a_fields.push((r, c));

                    row.push(1);
                } else {
                    row.push(val as u32 - 'a' as u32 + 1)
                }
            }

            field.push(row);
        }

        for r in 0..field.len() {
            println!("{:?}", field[r]);
        }
    }

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for r in 0..field.len() {
        if let Some(curr_row) = field.get(r) {
            for c in 0..curr_row.len() {
                let curr_field_val = field[r][c];
                let mut accessible_neighbours: Vec<(usize, usize)> = Vec::new();

                // up
                if r > 0 {
                    if field[r - 1][c] as i32 - curr_field_val as i32 <= 1 {
                        accessible_neighbours.push((r - 1, c));
                    }
                }

                // down
                if r + 1 < field.len() {
                    if field[r + 1][c] as i32 - curr_field_val as i32 <= 1 {
                        accessible_neighbours.push((r + 1, c));
                    }
                }

                // left
                if c > 0 {
                    if field[r][c - 1] as i32 - curr_field_val as i32 <= 1 {
                        accessible_neighbours.push((r, c - 1));
                    }
                }

                // right
                if c + 1 < curr_row.len() {
                    if field[r][c + 1] as i32 - curr_field_val as i32 <= 1 {
                        accessible_neighbours.push((r, c + 1));
                    }
                }

                graph.insert((r, c), accessible_neighbours);
            }
        }
    }

    println!("{:?}", graph);
    println!("{:?}", start);
    println!("{:?}", end);
    println!("{:?}", a_fields);

    let mut shortest_path_len: Option<usize> = None;
    for a_field in a_fields {
        let curr_shortest_path_len = bfs(&a_field, &end, &graph);
        if curr_shortest_path_len > 0 {
            println!("Current shortest path: {}", curr_shortest_path_len);

            if let Some(comp) = shortest_path_len {
                if comp > curr_shortest_path_len {
                    shortest_path_len = Some(curr_shortest_path_len);    
                }
            } else {
                shortest_path_len = Some(curr_shortest_path_len);
            }
        }
    }

    println!("The length of the shortest path: {}", shortest_path_len.unwrap());    

}

fn bfs(start: &(usize, usize), end: &(usize, usize), graph: &HashMap<(usize, usize), Vec<(usize, usize)>>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start.clone());

    let mut paths: VecDeque<((usize, usize), Vec<(usize, usize)>)> = VecDeque::new();
    paths.push_back((start.clone(), vec![start.clone()]));

    while let Some(curr_path) = paths.pop_front() {
        if let Some(neighbors) = graph.get(&curr_path.0) {
            for neighbor in neighbors {
                if visited.contains(neighbor) {
                    continue;
                } else {
                    if *neighbor == *end {
                        //println!("{:?}", curr_path.1);

                        return curr_path.1.len();
                    } else {
                        visited.insert(neighbor.clone());
                        let mut new_path = curr_path.1.clone();
                        new_path.push(neighbor.clone());
                        paths.push_back((neighbor.clone(), new_path));
                    }
                }
            }
        }
    }

    return 0;
}