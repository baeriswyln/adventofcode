use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Operator {
    Plus,
    PlusSelf,
    Times,
    TimesSelf,
}

#[derive(Clone)]
struct Monk {
    items: VecDeque<u128>,
    operation: Operator,
    factor: usize,
    modulo: usize,
    if_true: usize,
    if_false: usize,
    counter: usize,
}

#[derive(Clone)]
pub struct Monkey {
    data: Vec<Monk>,
}

impl crate::Advent for Monkey {
    fn new(data: &str) -> Self {
        Monkey {
            data: data.split("\n\n").map(|l| {
                let mut monkey_iter = l.lines();
                monkey_iter.next();

                // items
                let line_iter = monkey_iter.next().unwrap().split([' ', ',']);
                let mut items = VecDeque::new();
                line_iter.for_each(|l| match l.parse() {
                    Ok(v) => items.push_back(v),
                    Err(_) => ()
                });

                // operation
                let mut line_iter = monkey_iter.next().unwrap().split(" ");
                let operation = match line_iter.nth(6).unwrap() {
                    "+" => match line_iter.next().unwrap() {
                        "old" => (Operator::PlusSelf, 0),
                        v => (Operator::Plus, v.parse().unwrap())
                    },
                    _ => match line_iter.next().unwrap() {
                        "old" => (Operator::TimesSelf, 0),
                        v => (Operator::Times, v.parse().unwrap())
                    }
                };

                // division test
                let modulo = monkey_iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();

                // true/false action
                let if_true = monkey_iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();
                let if_false = monkey_iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();

                Monk {
                    items,
                    operation: operation.0,
                    factor: operation.1,
                    modulo,
                    if_true,
                    if_false,
                    counter: 0,
                }
            }).collect()
        }
    }

    fn year() -> u16 { 2022 }

    fn day() -> u8 {
        11
    }

    fn part1(&self) -> usize {
        let rounds = 20;
        let monkeys = do_iterations(self.data.clone(), rounds, 3);

        let mut values = monkeys.iter().map(|l| l.counter).collect::<Vec<usize>>();
        values.sort();
        values.reverse();

        (values[0] * values[1]) as usize
    }

    fn part2(&self) -> usize {
        let rounds = 10000;
        let monkeys = do_iterations(self.data.clone(), rounds, 1);
        println!("{:?}", monkeys[0].items);

        let mut values = monkeys.iter().map(|l| l.counter).collect::<Vec<usize>>();
        values.sort();
        values.reverse();

        println!("{:?}", values);
        (values[0] * values[1]) as usize
    }
}

fn do_iterations(mut monkeys: Vec<Monk>, rounds: usize, divide_factor: usize) -> Vec<Monk> {
    let modulus = 10000 * monkeys.iter().map(|m| m.modulo).product::<usize>() as u128;

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            for item in monkey.items {
                let mut val = item;
                // println!("Inspect {}", val);
                // println!("Inspect {}", monkey.factor);
                match monkey.operation {
                    Operator::Plus => val += monkey.factor as u128,
                    Operator::PlusSelf => val *= 2,
                    Operator::Times => val *= monkey.factor as u128,
                    Operator::TimesSelf => val *= val,
                }
                // println!("Value is now {}", val);
                val %= modulus;
                val /= divide_factor as u128;
                // println!("Value is now {}", val);
                if val % monkey.modulo as u128 == 0 {
                    monkeys[monkey.if_true].items.push_back(val);
                } else {
                    monkeys[monkey.if_false].items.push_back(val);
                }
                monkey.counter += 1;
            }
            monkey.items = VecDeque::new();
            monkeys[i] = monkey;
        }

        // for monkey in &monkeys {
        //     println!("Monkey {}: {:?}", 0, monkeys[0].items);
        // }
        // println!();
    }
    monkeys
}
