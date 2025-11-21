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

fn add_badge_score(score_sum: &mut u32, group_pack_content: &[String; 3]) {
    for t in group_pack_content[0].chars() {
        if group_pack_content[1].contains(t) && group_pack_content[2].contains(t) {
            *score_sum += get_priority(Some(t)) as u32;

            println!("{:?}", t);
            break;
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        // Part1

        //let score_sum: u32 = lines.map_while(Result::ok).map(|l| find_duplicate_type(l)).map(|ito| get_priority(ito) as u32).sum();

        //println!("The sum of priorities: {score_sum}");

        // Part 2

        let mut line_in_group = 0;
        let mut group_pack_content: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
        let mut score_sum = 0u32;
        for line in lines.map_while(Result::ok) {
            if line_in_group == 3 {
                println!("{:?}", group_pack_content);

                add_badge_score(&mut score_sum, &group_pack_content);

                line_in_group = 0;
            }

            group_pack_content[line_in_group] = line;
            line_in_group += 1;
        }

        // Process last group
        add_badge_score(&mut score_sum, &group_pack_content);

        println!("The sum of priorities of badges: {score_sum}");
    }
}
