use aoc22_shared::*;

#[derive(Debug)]
struct Sensor {
    pos_x: i32,
    pos_y: i32,
    closest_b_dist: i32
}

impl Sensor {
    fn in_reach(&self, x: i32, y: i32) -> bool {
        let distance = (self.pos_x - x).abs() + (self.pos_y - y).abs();

        return distance <= self.closest_b_dist;
    }
}

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut sensors: Vec<Sensor> = Vec::new();
        let mut beacons: Vec<(i32, i32)> = Vec::new();

        let mut left_end: Option<i32> = None;
        let mut right_end: Option<i32> = None;
        
        for line in lines.map_while(Result::ok) {
            let mut sensor_x: i32 = 0;
            let mut sensor_y: i32 = 0;
            let mut closest_beacon_x: i32 = 0;
            let mut closest_beacon_y: i32 = 0;

            for (i, line_part) in line.split(": ").enumerate() {
                if i == 0 {
                    for (j, sensor_pos_part) in line_part.split(", ").enumerate() {
                        if j == 0 {
                            sensor_x = sensor_pos_part[12..].parse::<i32>().unwrap();
                        } else if j == 1 {
                            sensor_y = sensor_pos_part[2..].parse::<i32>().unwrap();
                        } else {
                            panic!("Sensor position string should only have 2 parts'")
                        }
                    }
                } else if i == 1 {
                    for (j, beacon_pos_part) in line_part.split(", ").enumerate() {
                        if j == 0 {
                            closest_beacon_x = beacon_pos_part[23..].parse::<i32>().unwrap();
                        } else if j == 1 {
                            closest_beacon_y = beacon_pos_part[2..].parse::<i32>().unwrap();
                        } else {
                            panic!("Beacon position string should only have 2 parts'")
                        }
                    }
                } else {
                    panic!("Line should only have 2 parts!");
                }
            }

            beacons.push((closest_beacon_x, closest_beacon_y));

            let distance = (closest_beacon_x - sensor_x).abs() + (closest_beacon_y - sensor_y).abs();

            sensors.push(Sensor{pos_x: sensor_x, pos_y: sensor_y, closest_b_dist: distance});

            let sensor_left_reach = sensor_y - distance - 1;
            let sensor_right_reach = sensor_y + distance + 1;

            match left_end {
                None => {
                    left_end = Some(sensor_left_reach);
                },
                Some(val) => {
                    if sensor_left_reach < val {
                        left_end = Some(sensor_left_reach);
                    }
                }
            }
            match right_end {
                None => {
                    right_end = Some(sensor_right_reach);
                },
                Some(val) => {
                    if sensor_right_reach > val {
                        right_end = Some(sensor_right_reach);
                    }
                }
            }
        }

        println!("{:?}", sensors);

        let mut fields_in_reach: usize = 0;

        let y = 2000000;
        for x in (left_end.unwrap()..=right_end.unwrap()) {
            for sensor in &sensors {
                if sensor.in_reach(x, y) && !beacons.contains(&(x, y)) {
                    fields_in_reach += 1;
                    break;
                }
            }
        }

        println!("Number of fields that can't have a beacon: {fields_in_reach} in row: {y}");
    }
}
