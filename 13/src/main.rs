use aoc22_shared::*;
use serde_json::Value;

fn main() {
    if let Ok(lines) = read_lines("res/example1.txt") {
        let mut right: Option<Value> = None;
        let mut left: Option<Value> = None;
        for line in lines.map_while(Result::ok) {
            if !line.is_empty() {
                println!("{line}");

                let val: Value = match serde_json::from_str(&line) {
                        Ok(res) => res,
                        _ => std::process::exit(1)
                    };
                if left.is_none() {
                    println!("val = {:?}", val);
                    left = Some(val);
                } else {
                    println!("val = {:?}", val);
                    right = Some(val);
                }
            } else {
                println!("left = {:?}", left);
                println!("right = {:?}", right);

                right = None;
                left = None;
            }
        }
    }
}
