use aoc22_shared::*;

fn find_duplicate_type(line: String) -> Option<char> {
    let first_comp = &line[..line.len()/2];
    let second_comp = &line[line.len()/2..];

    for first_type in first_comp.chars() {
        if second_comp.contains(first_type) {
            return Some(first_type);
        }
    };

    return None;
}

fn get_priority(item_type_opt: Option<char>) -> u8 {
    match item_type_opt {
        Some(item_type) => {
            let ascii = item_type as u8;
            if ascii > 96 {
                ascii - 97 + 1
            } else {
                ascii - 65 + 27
            }
        }
        None => 0
    }
}

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let score_sum: u32 = lines.map_while(Result::ok).map(|l| find_duplicate_type(l)).map(|ito| get_priority(ito) as u32).sum();

        println!("The sum of priorities: {score_sum}")
    }
}
