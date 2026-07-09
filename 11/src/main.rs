struct Monkey {
    worry_levels: Vec<u64>,
    operation: fn(u64) -> u64,
    test_by: u64,
    target: (usize, usize)
}

fn main() {
    let mut monkeys: [Monkey;8] = [
        Monkey{worry_levels: vec![92, 73, 86, 83, 65, 51, 55, 93], operation: |x| x * 5, test_by: 11, target: (3, 4)},
        Monkey{worry_levels: vec![99, 67, 62, 61, 59, 98], operation: |x| x * x, test_by: 2, target: (6, 7)},
        Monkey{worry_levels: vec![81, 89, 56, 61, 99], operation: |x| x * 7, test_by: 5, target: (1, 5)},
        Monkey{worry_levels: vec![97, 74, 68], operation: |x| x + 1, test_by: 17, target: (2, 5)},
        Monkey{worry_levels: vec![78, 73], operation: |x| x + 3, test_by: 19, target: (2, 3)},
        Monkey{worry_levels: vec![50], operation: |x| x + 5, test_by: 7, target: (1, 6)},
        Monkey{worry_levels: vec![95, 88, 53, 75], operation: |x| x + 8, test_by: 3, target: (0, 7)},
        Monkey{worry_levels: vec![50, 77, 98, 85, 94, 56, 89], operation: |x| x + 2, test_by: 13, target: (4, 0)}
    ];

    let mut monkeys_inspections_count: [usize;8] = [0;8];

    // rounds
    for r in 0..20 {
        println!("Round: {r}");

        for m in 0..8 {
            let curr_worry_levels = monkeys[m].worry_levels.clone();
            let curr_inspections_count = monkeys_inspections_count[m];
            monkeys_inspections_count[m] = curr_inspections_count + curr_worry_levels.len();

            for wl in curr_worry_levels {
                let worry_level_after_inspection = (monkeys[m].operation)(wl) / 3;
                let target: usize;
                if worry_level_after_inspection % monkeys[m].test_by == 0{
                    target = monkeys[m].target.0;
                } else {
                    target = monkeys[m].target.1;
                }
                monkeys[target].worry_levels.push(worry_level_after_inspection);
            }
            
            monkeys[m].worry_levels = vec![];
        }
    }

    println!("Monkeys inspected stuff this many times: {:?}", monkeys_inspections_count);

    let mut most_active: usize = 0;
    let mut second_most_active: usize = 0;
    for ic in monkeys_inspections_count {
        if ic >= most_active {
            second_most_active = most_active;
            most_active = ic;
        } else if ic > second_most_active {
            second_most_active = ic;
        }
    }

    println!("The level of monkey business: {}", most_active * second_most_active);
}
