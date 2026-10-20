use std::cmp::max_by_key;

use aoc22_shared::*;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut field: Vec<Vec<bool>> = vec![vec![false; 1000]; 200];
        let mut max_row: usize = 0;

        for line in lines.map_while(Result::ok) {
            let mut start: Option<(usize, usize)> = None;
            for point in line.split(" -> ") {
                println!("{point}");

                let point_split: Vec<&str> = point.split(",").collect();
                let p_x: usize = point_split[0].parse::<usize>().expect("Could not convert x coordinate of point: {point}");
                let p_y: usize = point_split[1].parse().expect("Could not convert y coordinate of point: {point}");

                if p_y > max_row {
                    max_row = p_y;
                }

                match start {
                    Some(s) => {
                        if s.0 == p_x {
                            let mut y_range = s.1..=p_y;
                            if s.1 > p_y {
                                y_range = p_y..=s.1;
                            }
                            for y in y_range {
                                field[y][p_x] = true;
                            }
                        } else if s.1 == p_y {
                            let mut x_range = s.0..=p_x;
                            if s.0 > p_x {
                                x_range = p_x..=s.0;
                            }
                            for x in x_range {
                                field[p_y][x] = true;
                            }
                        } else {
                            panic!("Segments should be horizontal or vertical lines!")
                        }
                    },
                    None => {
                    }
                }

                start = Some((p_x, p_y))
            }
        }

        print_field(&field, 0, 999);

        let heighest_available_row = max_row + 1;

        println!("Ground level: {}", heighest_available_row + 1);

        let mut first_col: usize = 500;
        let mut last_col: usize = 500;

        let sand_start_pos: (usize, usize) = (500, 0);
        let mut sand_count = 0;
        let mut need_more_sand = true;
        while need_more_sand {
            if !field[sand_start_pos.1][sand_start_pos.0] {
                sand_count += 1;
                let mut sand_curr_pos: (usize, usize) = sand_start_pos;
                let mut sand_moving = true;
                while sand_moving {
                    // try next move
                    if sand_curr_pos.1 < heighest_available_row {
                        if !field[sand_curr_pos.1 + 1][sand_curr_pos.0] {
                            // try down

                            sand_curr_pos = (sand_curr_pos.0, sand_curr_pos.1 + 1)

                        } else if sand_curr_pos.0 > 0 && !field[sand_curr_pos.1 + 1][sand_curr_pos.0 - 1]{
                            // try left

                            sand_curr_pos = (sand_curr_pos.0 - 1, sand_curr_pos.1 + 1)

                        } else if sand_curr_pos.0 < 999 && !field[sand_curr_pos.1 + 1][sand_curr_pos.0 + 1]{
                            // try right

                            sand_curr_pos = (sand_curr_pos.0 + 1, sand_curr_pos.1 + 1)
                        } else {
                            sand_moving = false;
                        }
                    } else {
                        sand_moving = false;
                    }
                }

                field[sand_curr_pos.1][sand_curr_pos.0] = true;

                if sand_curr_pos.0 < first_col {
                    first_col = sand_curr_pos.0;
                }

                if sand_curr_pos.0 > last_col {
                    last_col = sand_curr_pos.0;
                }
            } else {
                need_more_sand = false;
            }
        }

        print_field(&field, first_col, last_col);

        println!("Sand particles generated: {sand_count}");

    }
}

fn print_field(field: &Vec<Vec<bool>>, first_col: usize, last_col: usize) {
    for r in field {
        for c in first_col..=last_col {
            if r[c] {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}