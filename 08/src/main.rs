use aoc22_shared::*;

fn main() {
    let mut trees: Vec<Vec<(bool, u32)>> = Vec::new();

    if let Ok(lines) = read_lines("res/input.txt") {
        for line in lines.map_while(Result::ok) {
            let mut tree_line: Vec<(bool, u32)> = Vec::new();

            for c in line.chars() {
                let height = c.to_digit(10).unwrap();
                tree_line.push((false, height));
            }

            trees.push(tree_line);
        }
    }

    println!("{:?}", trees);

    // from top to bottom
    for col in 0..trees[0].len() {
        let mut max_height: Option<u32> = None;

        for row in 0..trees.len() {
            update_tree(trees.get_mut(row).unwrap().get_mut(col).unwrap(), &mut max_height);

            if max_height == Some(9) {
                break;
            }
        }
    }

    // from bottom to top
    for col in 0..trees[0].len() {
        let mut max_height: Option<u32> = None;

        for row in (1..trees.len()).rev() {
            update_tree(trees.get_mut(row).unwrap().get_mut(col).unwrap(), &mut max_height);
            
            if max_height == Some(9) {
                break;
            }
        }
    }

    // from left to right
    for row in 0..trees.len() {
        let mut max_height: Option<u32> = None;

        for col in 0..trees[row].len() {
            update_tree(trees.get_mut(row).unwrap().get_mut(col).unwrap(), &mut max_height);

            if max_height == Some(9) {
                break;
            }    
        }
    }

    // from left to right
    for row in 0..trees.len() {
        let mut max_height: Option<u32> = None;

        for col in (1..trees[row].len()).rev() {
            update_tree(trees.get_mut(row).unwrap().get_mut(col).unwrap(), &mut max_height);
            
            if max_height == Some(9) {
                break;
            }
        }
    }

    for tree_line in trees.iter() {
        println!("{:?}", tree_line);
    }

    let mut visible_count: u32 = 0;
    for tree_line in trees.iter() {
        for tree in tree_line.iter() {
            if tree.0 {
                visible_count += 1;
            }
        }
    }

    println!("{visible_count} trees are visible.");

    let mut max_scenic_score: u32 = 0;
    for r in 0..trees.len() {
        for c in 0..trees[r].len() {
            let curr_scenic_score = get_scenic_score(r, c, &trees);
            if curr_scenic_score > max_scenic_score {
                max_scenic_score = curr_scenic_score;
            }
        }
    }

    println!("Max scenic score: {max_scenic_score}");
}

fn update_tree(tree: &mut (bool, u32), max_height: &mut Option<u32>) {
    if let Some(mh) = max_height {
        if tree.1 > *mh {
            tree.0 = true;
            *max_height = Some(tree.1);
        }
    } else {
        tree.0 = true;
        *max_height = Some(tree.1);
    }
}

fn get_scenic_score(tree_row: usize, tree_col: usize, trees: &Vec<Vec<(bool, u32)>>) -> u32 {
    let curr_height = trees[tree_row][tree_col].1;

    let mut visible_left: u32 = 0;
    if tree_col > 0 {
        for c in (0..tree_col).rev() {
            visible_left += 1;

            if trees[tree_row][c].1 >= curr_height {
                break;
            }
        }
    }

    let mut visible_right: u32 = 0;
    if tree_col < trees[tree_row].len() - 1 {
        for c in tree_col + 1..trees[tree_row].len() {
            visible_right += 1;

            if trees[tree_row][c].1 >= curr_height {
                break;
            }
        }
    }

    let mut visible_up: u32 = 0;
    if tree_row > 0 {
        for r in (0..tree_row).rev() {
            visible_up += 1;

            if trees[r][tree_col].1 >= curr_height {
                break;
            }
        }
    }

    let mut visible_down: u32 = 0;
    if tree_row < trees.len() - 1 {
        for r in tree_row + 1..trees.len() {
            visible_down += 1;

            if trees[r][tree_col].1 >= curr_height {
                break;
            }
        }
    }

    println!("Row: {tree_row}, col: {tree_col} scores: {visible_up}, {visible_down}, {visible_left}, {visible_right}");

    return visible_up * visible_down * visible_left * visible_right;
}
