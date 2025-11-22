use aoc22_shared::*;

fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let mut columns: [String; 9] = [String::new(), String::new(), String::new(),
                                        String::new(), String::new(), String::new(),
                                        String::new(), String::new(), String::new()];

        for (line_num, line) in lines.map_while(Result::ok).enumerate() {
            if line_num < 8 {
                for i in 0..9 {
                    let co = line.chars().nth(1+i*4);
                    if let Some(c) = co && c != ' ' {
                        columns[i].push(c);
                    } 
                }
            }

            if line_num == 8 {
                println!("{:?}", columns)
            }

            if line_num > 9 {
                let parts = line.split(" ").collect::<Vec<&str>>();
                
                let crate_count = parts[1].parse::<usize>().unwrap();
                let from = parts[3].parse::<usize>().unwrap() - 1;
                let to = parts[5].parse::<usize>().unwrap() - 1;

                // Part 1
                //columns[to] = format!("{}{}",&columns[from][..crate_count].chars().rev().collect::<String>(),columns[to]);
                // Part 2
                columns[to] = format!("{}{}",&columns[from][..crate_count],columns[to]);
                columns[from] = String::from(&columns[from][crate_count..]);

                println!("{:?}", columns)
            }
        }

        for column in &columns {
            print!("{}", &column.chars().nth(0).unwrap_or_default());
        }
        print!("\n");
    }
}
