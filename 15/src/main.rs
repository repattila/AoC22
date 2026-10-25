use aoc22_shared::*;

#[derive(Debug)]
struct Sensor {
    pos_x: i32,
    pos_y: i32,
    closest_b_dist_2: i32
}

fn main() {
    if let Ok(lines) = read_lines("res/example1.txt") {
        let mut sensors: Vec<Sensor> = Vec::new();
        
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

            let distance = (closest_beacon_x - sensor_x).pow(2) + (closest_beacon_y - sensor_y).pow(2);

            sensors.push(Sensor{pos_x: sensor_x, pos_y: sensor_y, closest_b_dist_2: distance});
        }

        println!("{:?}", sensors);
    }
}
