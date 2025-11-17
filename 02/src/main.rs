use aoc22_shared::*;

enum Sign {
    Rock,
    Paper,
    Scissors,
    NA
}

struct Game {
    op_sign: Sign,
    my_sign: Sign
}

impl Game {
    fn new(line: String) -> Game {
        let mut line_chars = line.chars();
        let op_sign_char = line_chars.next().unwrap();
        line_chars.next();
        let my_sign_char = line_chars.next().unwrap();

        let op_sign: Sign = match op_sign_char {
            'A' => Sign::Rock,
            'B' => Sign::Paper,
            'C' => Sign::Scissors,
            _ => Sign::NA
        };

        let my_sign: Sign = match my_sign_char {
            'X' => Sign::Rock,
            'Y' => Sign::Paper,
            'Z' => Sign::Scissors,
            _ => Sign::NA
        };

        Game{op_sign, my_sign}
    }

    fn evaluate(&self) -> u8 {
        let my_sign_score = match self.my_sign {
            Sign::Rock => 1u8,
            Sign::Paper => 2u8,
            Sign::Scissors => 3u8,
            Sign::NA => 0u8,
        };

        let outcome_score = match self.my_sign {
            Sign::Rock => {
                match self.op_sign {
                    Sign::Rock => 3u8,
                    Sign::Paper => 0u8,
                    Sign::Scissors => 6u8,
                    Sign::NA => 0u8,
                }
            },
            Sign::Paper => {
                match self.op_sign {
                    Sign::Rock => 6u8,
                    Sign::Paper => 3u8,
                    Sign::Scissors => 0u8,
                    Sign::NA => 0u8,
                }
            },
            Sign::Scissors => {
                match self.op_sign {
                    Sign::Rock => 0u8,
                    Sign::Paper => 6u8,
                    Sign::Scissors => 3u8,
                    Sign::NA => 0u8,
                }
            },
            Sign::NA => 0u8,
        };

        my_sign_score + outcome_score
    }
}


fn main() {
    if let Ok(lines) = read_lines("res/input.txt") {
        let score_sum: u32 = lines.map_while(Result::ok).map(Game::new).map(|g| g.evaluate() as u32).sum();

        println!("The sum of scores: {score_sum}")
    }
}
