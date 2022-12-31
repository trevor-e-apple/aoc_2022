use regex::Regex;
use std::{env, fs::read_to_string, process};

#[derive(Debug, Default)]
enum Operation {
    #[default]
    Add,
    Multiply,
}

#[derive(Debug, Default)]
enum Operand {
    #[default]
    Old,
    Num(i32),
}

#[derive(Debug, Default)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    operand: Operand,
    test_divisor: i32,
    true_throw_to: i32,
    false_throw_to: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = &args[1];

    let contents = read_to_string(path).unwrap();

    // parse input
    let mut monkeys: Vec<Monkey> = Vec::new();
    let reggie = Regex::new(concat!(
        r"Monkey (?P<monkey_index>\d+):\s*",
        r"\s*Starting items:\s*(?P<items>(?:\d+,*\s*)+)\s*",
        r"\s*Operation: new = old\s*(?P<operator>\*|\+)\s*(?P<operand>\d+|old)\s*",
        r"\s*Test: divisible by (?P<divisor>\d+)\s*",
        r"\s*If true: throw to monkey (?P<true_throw_to>\d+)\s*",
        r"\s*If false: throw to monkey (?P<false_throw_to>\d+)\s*"
    ))
    .unwrap();
    for (index, monkey_text) in reggie.captures_iter(&contents).enumerate() {
        println!("index: {}", &monkey_text["monkey_index"]);
        println!("items: {}", &monkey_text["items"]);
        println!("operator: {}", &monkey_text["operator"]);
        println!("operand: {}", &monkey_text["operand"]);
        println!("test divisor: {}", &monkey_text["divisor"]);
        println!("true throw to: {}", &monkey_text["true_throw_to"]);
        println!("false throw to: {}", &monkey_text["false_throw_to"]);
        println!("");
        let mut monkey = Monkey {
            ..Default::default()
        };
        for item_str in monkey_text["items"].split(",") {
            let item_str = item_str.to_owned();
            monkey.items.push(item_str.trim().parse().unwrap());
        }
        monkey.operation = match &monkey_text["operator"] {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => {
                assert!(false);
                Operation::Add
            }
        };
        monkey.operand = match &monkey_text["operand"] {
            "old" => Operand::Old,
            other => Operand::Num(other.parse().unwrap()),
        };
        monkey.test_divisor = monkey_text["divisor"].parse().unwrap();
        monkey.true_throw_to = monkey_text["true_throw_to"].parse().unwrap();
        monkey.false_throw_to = monkey_text["false_throw_to"].parse().unwrap();

        assert!(monkey.true_throw_to != (index as i32));
        assert!(monkey.false_throw_to != (index as i32));

        println!("monkey: {:?}", monkey);
        monkeys.push(monkey);
    }

    // simulate for 20 rounds
    for round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            // update the worry levels
            let monkey = monkeys.get(monkey_index).unwrap();
            // queue up what to add to the other monkeys
            let mut command_buffer: Vec<(i32, i32)> = Vec::new();
            for item in monkey.items.as_slice() {
                let operand = match monkey.operand {
                    Operand::Old => *item,
                    Operand::Num(value) => value
                };
                let mut value = match monkey.operation {
                    Operation::Add => item + operand,
                    Operation::Multiply => item * operand,
                };
                value /= 3;
                let throw_to = if value % monkey.test_divisor == 0 {
                    monkey.true_throw_to
                } else {
                    monkey.false_throw_to
                };
                command_buffer.push((throw_to, value));
            };

            // update the other monkeys
            for (throw_to, value) in command_buffer {
                let throw_to = monkeys.get_mut(throw_to as usize).unwrap();
                throw_to.items.push(value);
            }

            // remove all the old items from this monkey at the end of the loop
            // (assume no monkey throws to itself under any conditions)
            let monkey = monkeys.get_mut(monkey_index).unwrap();
            monkey.items = Vec::new();
        }

        println!("After round {:?}", round);
        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys.get(monkey_index).unwrap();
            println!("Monkey {:?}: {:?}", monkey_index, monkey.items);
        }
    }
}
