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

    println!("{visible_count} trees are visible.")
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
