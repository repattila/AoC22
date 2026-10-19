use aoc22_shared::*;

fn main() {
    let mut field: Vec<Vec<bool>> = vec![vec![false; 200]; 200];

    if let Ok(lines) = read_lines("res/input.txt") {
        for line in lines.map_while(Result::ok) {
            let mut start: Option<(usize, usize)> = None;
            for point in line.split(" -> ") {
                println!("{point}");

                let point_split: Vec<&str> = point.split(",").collect();
                let p_x: usize = point_split[0].parse::<usize>().expect("Could not convert x coordinate of point: {point}") - 400;
                let p_y: usize = point_split[1].parse().expect("Could not convert y coordinate of point: {point}");

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

        print_field(&field);

        let sand_start_pos: (usize, usize) = (100, 0);
        let mut sand_count = 0;
        let mut need_more_sand = true;
        while need_more_sand {
            sand_count += 1;
            let mut sand_curr_pos: (usize, usize) = sand_start_pos;
            let mut sand_moving = true;
            while sand_moving {
                // try next move
                if sand_curr_pos.1 < 199 {
                    if !field[sand_curr_pos.1 + 1][sand_curr_pos.0] {
                        // try down

                        sand_curr_pos = (sand_curr_pos.0, sand_curr_pos.1 + 1)

                    } else if sand_curr_pos.0 > 0 && !field[sand_curr_pos.1 + 1][sand_curr_pos.0 - 1]{
                        // try left

                        sand_curr_pos = (sand_curr_pos.0 - 1, sand_curr_pos.1 + 1)

                    } else if sand_curr_pos.0 < 199 && !field[sand_curr_pos.1 + 1][sand_curr_pos.0 + 1]{
                        // try right

                        sand_curr_pos = (sand_curr_pos.0 + 1, sand_curr_pos.1 + 1)
                    } else {
                        sand_moving = false;
                    }
                } else {
                    sand_moving = false;
                    need_more_sand = false;
                }
            }

            field[sand_curr_pos.1][sand_curr_pos.0] = true;
        }

        print_field(&field);

        println!("Sand particles generated: {sand_count}");

    }
}

fn print_field(field: &Vec<Vec<bool>>) {
    for r in field {
        for p in r {
            if *p {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}