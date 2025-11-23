use aoc22_shared::*;

fn insert_next_char(window: &mut[char; 4], next_char: char) {
    for i in 0..3 {
        window[i] = window[i+1]
    }
    window[3] = next_char;
}

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let line = lines.map_while(Result::ok).last().unwrap_or_default();

        let mut window: [char; 4] = [' '; 4];
        let mut pos: usize = 0;

        for ch in line.chars() {
            insert_next_char(&mut window, ch);

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
