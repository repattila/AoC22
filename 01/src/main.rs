use aoc22_shared::*;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut max = 0;
        let mut curr = 0;

        for line in lines.map_while(Result::ok) {
            if line.is_empty() {
                if curr > max {
                    max = curr;
                }

                curr = 0;
            } else {
                curr += line.parse::<u32>().unwrap()
            }
        }

        println!("Max calories: {max}");
    }
}
