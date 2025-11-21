use aoc22_shared::*;

struct Range(u8, u8);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.0 || self.0 >= other.0 && self.0 <= other.1
    }

    fn new(p: &str) -> Range {
        let separator_pos = p.find('-').unwrap();
        Range(*(&p[..separator_pos].parse::<u8>().unwrap()), *(&p[separator_pos+1..].parse::<u8>().unwrap()))
    }
}

fn process_line(line: &String) -> u8 {
    let separator_pos = line.find(',').unwrap();
    let r1 = Range::new(&line[..separator_pos]);
    let r2 = Range::new(&line[separator_pos+1..]);
    // Part 1
    //if r1.contains(&r2) || r2.contains(&r1) {
    if r1.overlaps(&r2) {
        1
    } else {
        0
    }
}

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let score_sum: u32 = lines.map_while(Result::ok).map(|l| process_line(&l) as u32).sum();

        println!("Number of overlapping ranges: {score_sum}");
    }
}
