use aoc22_shared::*;
use std::collections::HashSet;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut h_pos = (0, 0);
        let mut knots_pos = [(0, 0); 9];
        let mut t_pos_set: HashSet<(i32, i32)> = HashSet::new();

        for line in lines.map_while(Result::ok) {
            println!("{line}");

            let mut split_line = line.split(" ");

            let s: (i32, i32);
            match split_line.next() {
                Some("U") => {
                    s = (0, 1);
                }
                Some("D") => {
                    s = (0, -1);
                }
                Some("R") => {
                    s = (1, 0);
                }
                Some("L") => {
                    s = (-1, 0);
                }
                Some(c) => panic!("Unexpected direction: {c}"),
                None => panic!("Command line doesn't have a command")
            }

            let steps = split_line.next().unwrap().parse::<i32>().unwrap();
            for _ in 0..steps {
                h_pos.0 += s.0;
                h_pos.1 += s.1;

                let mut prev_knot_pos = h_pos;
                for i in 0..9 {
                    let mut curr_knot_pos = knots_pos[i];

                    let x_diff = prev_knot_pos.0 - curr_knot_pos.0;
                    let y_diff = prev_knot_pos.1 - curr_knot_pos.1;

                    if x_diff == 2 && y_diff == 0 {
                        curr_knot_pos.0 += 1;
                    } else if x_diff == -2 && y_diff == 0 {
                        curr_knot_pos.0 -= 1;
                    } else if y_diff == 2 && x_diff == 0 {
                        curr_knot_pos.1 += 1;
                    } else if y_diff == -2 && x_diff == 0 {
                        curr_knot_pos.1 -= 1;
                    } else if x_diff == 2 && y_diff == 1 {
                        curr_knot_pos.0 += 1;
                        curr_knot_pos.1 += 1;
                    } else if x_diff == -2 && y_diff == 1 {
                        curr_knot_pos.0 -= 1;
                        curr_knot_pos.1 += 1;
                    } else if x_diff == 2 && y_diff == -1 {
                        curr_knot_pos.0 += 1;
                        curr_knot_pos.1 -= 1;
                    } else if x_diff == -2 && y_diff == -1 {
                        curr_knot_pos.0 -= 1;
                        curr_knot_pos.1 -= 1;
                    } else if x_diff == 1 && y_diff == 2 {
                        curr_knot_pos.0 += 1;
                        curr_knot_pos.1 += 1;
                    } else if x_diff == 1 && y_diff == -2 {
                        curr_knot_pos.0 += 1;
                        curr_knot_pos.1 -= 1;
                    } else if x_diff == -1 && y_diff == 2 {
                        curr_knot_pos.0 -= 1;
                        curr_knot_pos.1 += 1;
                    } else if x_diff == -1 && y_diff == -2 {
                        curr_knot_pos.0 -= 1;
                        curr_knot_pos.1 -= 1;
                    } else if x_diff == 2 && y_diff == 2 {
                        curr_knot_pos.0 += 1;
                        curr_knot_pos.1 += 1;
                    } else if x_diff == 2 && y_diff == -2 {
                        curr_knot_pos.0 += 1;
                        curr_knot_pos.1 -= 1;
                    } else if x_diff == -2 && y_diff == -2 {
                        curr_knot_pos.0 -= 1;
                        curr_knot_pos.1 -= 1;
                    } else if x_diff == -2 && y_diff == 2 {
                        curr_knot_pos.0 -= 1;
                        curr_knot_pos.1 += 1;
                    } else if (x_diff < 2 && x_diff > -2) && (y_diff < 2 && y_diff > -2) {

                    } else {
                        panic!("Unexpected position difference: ({x_diff}, {y_diff})")
                    }

                    knots_pos[i] = curr_knot_pos;
                    prev_knot_pos = curr_knot_pos;
                }

                t_pos_set.insert(knots_pos[8]);
            }

            println!("{:?}",h_pos);
            for k_pos in knots_pos {
                println!("{:?}",k_pos);
            }
        }

        println!("Number of distinct T positions: {}", t_pos_set.len());
    }
}
