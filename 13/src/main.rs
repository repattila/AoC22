use aoc22_shared::*;
use serde_json::Value;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut right: Option<Value> = None;
        let mut left: Option<Value> = None;
        let mut pair_index: usize = 1;
        let mut result: usize = 0;
        for line in lines.map_while(Result::ok) {
            println!("{line}");

            if !line.is_empty() {
                println!("{line}");

                let val: Value = match serde_json::from_str(&line) {
                        Ok(res) => res,
                        _ => std::process::exit(1)
                    };
                if left.is_none() {
                    //println!("val = {:?}", val);
                    left = Some(val);
                } else {
                    //println!("val = {:?}", val);
                    right = Some(val);
                }
            } else {
                println!("left = {:?}", left);
                println!("right = {:?}", right);

                let order = compare(&left.unwrap(), &right.unwrap());

                println!("Order of pair: {order}");

                if order == -1 {
                    result += pair_index;
                }

                pair_index += 1;
                right = None;
                left = None;
            }
        }

        println!("{result}")
    }
}

fn compare(left: &Value, right: &Value) -> i8 {
    if left.is_u64() && right.is_u64() {
        let lval = left.as_u64().unwrap();
        let rval = right.as_u64().unwrap();

        println!("Comparing numbers: {} and {}", lval, rval);

        if lval < rval {
            return -1;
        } else if lval == rval {
            return 0;
        } else {
            return 1;
        }
    } else if left.is_u64() {
        let lval_vec = vec![left.clone()];
        let new_left = Value::Array(lval_vec);

        return compare(&new_left, right);
    } else if right.is_u64() {
        let rval_vec = vec![right.clone()];
        let new_right = Value::Array(rval_vec);

        return compare(left, &new_right);
    } else {
        println!("Comparing arrays: {:?} and {:?}", left, right);

        let mut res: i8 = 0;
        let lval = left.as_array().unwrap();
        let rval = right.as_array().unwrap();
        for l in lval.iter().enumerate() {
            if res == 0 {
                if l.0 < rval.len() {
                    res = compare(l.1, rval.get(l.0).unwrap());
                } else {
                    res = 1;
                }
            } else {
                break;
            }
        }

        if res == 0 && lval.len() < rval.len() {
            res = -1;
        }

        return res;
    }
}
