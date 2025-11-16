use aoc22_shared::*;

fn insert_sum(top: &mut [u32], sum: u32) {
    let mut temp: u32;
    let mut to_push_down : u32 = 0;
    for s in top {
        if to_push_down > 0 {
            temp = *s;
            *s = to_push_down;
            to_push_down = temp;
        } else {
            if sum > *s {
                to_push_down = *s;
                *s = sum;
            }
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut top3: [u32; 3] = [0, 0, 0];
        let mut curr = 0;

        for line in lines.map_while(Result::ok) {
            if line.is_empty() {
                insert_sum(&mut top3, curr);

                curr = 0;
            } else {
                curr += line.parse::<u32>().unwrap()
            }
        }

        println!("Max calories: {}", top3[0]);

        let sum: u32 = top3.iter().sum();
        println!("Max calories of top3: {}", sum);
    }
}
