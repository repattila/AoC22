use aoc22_shared::*;

fn main() {
    let mut field: Vec<Vec<bool>> = vec![vec![false; 200]; 200];

    if let Ok(lines) = read_lines("res/example1.txt") {
        for line in lines.map_while(Result::ok) {
            let mut start: Option<(usize, usize)> = None;
            for point in line.split(" -> ") {
                println!("{point}");

                let point_split: Vec<&str> = point.split(",").collect();
                let p_x: usize = point_split[0].parse().expect("Could not convert x coordinate of point: {point}");
                let p_y: usize = point_split[1].parse().expect("Could not convert y coordinate of point: {point}");

                match start {
                    Some(s) => {
                        if s.0 == p_x {

                        } else if s.1 == p_y {

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
    }
}
