use aoc22_shared::*;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut x_reg: i32 = 1;
        let mut prev_x_reg: i32 = 1;
        let mut cycle_counter: u32 = 1;

        let mut got_20: bool = false;
        let mut x_at_20: i32 = 0;
        let mut got_60: bool = false;
        let mut x_at_60: i32 = 0;
        let mut got_100: bool = false;
        let mut x_at_100: i32 = 0;
        let mut got_140: bool = false;
        let mut x_at_140: i32 = 0;
        let mut got_180: bool = false;
        let mut x_at_180: i32 = 0;
        let mut got_220: bool = false;
        let mut x_at_220: i32 = 0;

        for line in lines.map_while(Result::ok) {
            let mut split_line = line.split(" ");

            match split_line.next() {
                Some("noop") => {
                    cycle_counter += 1;
                }
                Some("addx") => {
                    let change = split_line.next().unwrap().parse::<i32>().unwrap();
                    
                    cycle_counter += 2;
                    prev_x_reg = x_reg;
                    x_reg += change;
                }
                Some(c) => panic!("Unexpected direction: {c}"),
                None => panic!("Command line doesn't have a command")
            }

            if !got_20 {
                if cycle_counter == 20 {
                    x_at_20 = x_reg;
                    got_20 = true;
                } else if cycle_counter > 20 {
                    x_at_20 = prev_x_reg;
                    got_20 = true;
                }
            } else if !got_60 {
                if cycle_counter == 60 {
                    x_at_60 = x_reg;
                    got_60 = true;
                } else if cycle_counter > 60 {
                    x_at_60 = prev_x_reg;
                    got_60 = true;
                }
            } else if !got_100 {
                if cycle_counter == 100 {
                    x_at_100 = x_reg;
                    got_100 = true;
                } else if cycle_counter > 100 {
                    x_at_100 = prev_x_reg;
                    got_100 = true;
                }
            } else if !got_140 {
                if cycle_counter == 140 {
                    x_at_140 = x_reg;
                    got_140 = true;
                } else if cycle_counter > 140 {
                    x_at_140 = prev_x_reg;
                    got_140 = true;
                }
            } else if !got_180 {
                if cycle_counter == 180 {
                    x_at_180 = x_reg;
                    got_180 = true;
                } else if cycle_counter > 180 {
                    x_at_180 = prev_x_reg;
                    got_180 = true;
                }
            } else if !got_220 {
                if cycle_counter == 220 {
                    x_at_220 = x_reg;
                    got_220 = true;
                } else if cycle_counter > 220 {
                    x_at_220 = prev_x_reg;
                    got_220 = true;
                }
            }
        }

        println!("X at 20 is {x_at_20}");
        println!("X at 60 is {x_at_60}");
        println!("X at 100 is {x_at_100}");
        println!("X at 140 is {x_at_140}");
        println!("X at 180 is {x_at_180}");
        println!("X at 220 is {x_at_220}");

        println!("Signal strength: {}", x_at_20 * 20 + x_at_60 * 60 + x_at_100 * 100 + x_at_140 * 140 + x_at_180 * 180 + x_at_220 * 220)
    }
}
