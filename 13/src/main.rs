use std::cmp::Ordering;

use aoc22_shared::*;
use serde_json::Value;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        // part 1
        let mut right: Option<Value> = None;
        let mut left: Option<Value> = None;
        let mut pair_index: usize = 1;
        let mut result: usize = 0;
        // part2
        let mut packets: Vec<Packet> = Vec::new();

        for line in lines.map_while(Result::ok) {
            println!("{line}");

            if !line.is_empty() {
                println!("{line}");

                let val: Value = match serde_json::from_str(&line) {
                        Ok(res) => res,
                        Err(e) => panic!("Could not parse line as json! Error: {e}")
                    };

                // part 2
                packets.push(Packet{content: val.clone()});

                // part 1
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

        // part 1
        println!("Part 1 result: {result}");

        // part 2

        let m2 = Value::Array(vec![Value::Array(vec![Value::from(2)])]);
        let m6 = Value::Array(vec![Value::Array(vec![Value::from(6)])]);
        // Add markers
        packets.push(Packet{ content: m2.clone() });
        packets.push(Packet{ content: m6.clone() });

        packets.sort();

        let mut pos_m2: usize = 0;
        let mut pos_m6: usize = 0;
        for p in packets.iter().enumerate() {
            println!("{}", p.1.content);

            if p.1.content == m2 {
                pos_m2 = p.0 + 1;
            } else if p.1.content == m6 {
                pos_m6 = p.0 + 1;
            }
        }

        println!("Part 2 result: {}", pos_m2 * pos_m6);
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

#[derive(Eq)]
struct Packet {
    content: Value
}

impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        compare(&self.content, &other.content) == 0
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        match compare(&self.content, &other.content) {
            -1 => Some(Ordering::Less),
            0 => Some(Ordering::Equal),
            1 => Some(Ordering::Greater),
            _ => None
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match compare(&self.content, &other.content) {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _r => panic!("Unexpected comparison result: {}", _r)
        }
    }
}