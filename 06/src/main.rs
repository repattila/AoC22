use aoc22_shared::*;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let line = lines.map_while(Result::ok).last().unwrap_or_default();

        let mut window: [char; 4] = [' ', ' ', ' ', ' '];
        let mut pos: usize = 0;

        for ch in line.chars() {
            window[0] = window[1];
            window[1] = window[2];
            window[2] = window[3];
            window[3] = ch;

            pos += 1;
            
            let mut has_repeating: bool = false;
            for i in 0..3 {
                for j in i+1..4 {
                    if window[i] == window[j] {
                        has_repeating = true;
                        break;
                    }
                }
            }

            if pos > 3 && !has_repeating {
                println!("Marker pos: {pos}");
                break;
            }
        }
    }
}
