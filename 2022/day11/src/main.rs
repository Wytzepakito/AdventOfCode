use std::{fs, clone, env::current_dir};
use regex::Regex;

#[derive(Clone, Debug)]
struct Monkey {
    pub num:u8,
    pub items: Vec<u128>,
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub test_divis: u8,
    pub true_endpoint: u8,
    pub false_endpoint: u8,
    pub number_of_inspections: u32,
}


impl Monkey {
    pub fn new(num:u8, items: Vec<u128>, op:&str, arg1:&str, arg2:&str, test_divis:u8, true_endpoint: u8, false_endpoint: u8) -> Monkey {
        Monkey {
            num: num,
            items: items,
            op: op.to_owned(),
            arg1: arg1.to_owned(),
            arg2: arg2.to_owned(),
            test_divis: test_divis,
            true_endpoint: true_endpoint,
            false_endpoint: false_endpoint,
            number_of_inspections: 0,
        }
    }

    pub fn parse_get_one_num(line: &str) -> u8 {
        let re = Regex::new(r"\d+").expect("Failed to compile monkey num regex");
        let x = re.find(line).map(|x| x.as_str()).unwrap_or("").parse().unwrap();
        x
    }

    pub fn parse_items_line(line: &str) -> Vec<u128> {
        let re = Regex::new(r"\d+").expect("Failed to compile item regex");
        let mut items = re.find_iter(line).filter_map(|item| item.as_str().parse().ok()).collect();
        items
    }

    pub fn parse_op_args(line: &str) -> (&str, &str, &str) {
        let arg_re = Regex::new(r"\d+|old").expect("Failed to compile args regex");
        let mut args: Vec<&str> = arg_re.find_iter(line).filter_map(|arg| Some(arg.as_str())).collect();
        let op_re = Regex::new(r"\+|\-|/|\*").expect("Failed to compile operation regex"); 
        let op = op_re.find(line).map(|op| op.as_str()).unwrap_or("");

        (op, args[0], args[1])
    }


}

fn create_monkey_from_cache(line_cache: Vec<&str>) -> Monkey {
    let op_args = Monkey::parse_op_args(line_cache[2]);
    let mut monkey = Monkey::new(
        Monkey::parse_get_one_num(line_cache[0]),
        Monkey::parse_items_line(line_cache[1]),
        &op_args.0.to_owned(),
        &op_args.1.to_owned(),
        &op_args.2.to_owned(),
        Monkey::parse_get_one_num(line_cache[3]),
        Monkey::parse_get_one_num(line_cache[4]),
        Monkey::parse_get_one_num(line_cache[5]),
    );
    monkey
}

fn read_file(filename: &str) -> (Vec<Monkey>, u128)  {
    let mut line_cache: Vec<&str> = Vec::new();
    let mut file_string = fs::read_to_string(filename).expect("I cannot open this file");
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut gcn:u128 = 1;

    for line in file_string.lines() {
        if line.is_empty() {
            let monkey = create_monkey_from_cache(line_cache);
            gcn *= monkey.test_divis as u128;
            monkeys.push(monkey);
            line_cache = Vec::new();

        } else {
            line_cache.push(line);
        }
    }
    let monkey = create_monkey_from_cache(line_cache);
    gcn *= monkey.test_divis as u128;
    monkeys.push(monkey);
    (monkeys, gcn)
}

fn let_monkey_play(index: usize, monkeys: &mut Vec<Monkey>, gcn: u128) {
    let current_monkey = monkeys[index].clone();
    for item in current_monkey.items {
        let arg1: u128 = match current_monkey.arg1.as_str() {
            "old" => item,
            _ => monkeys[index].arg1.parse().unwrap()
        };
        let arg2: u128 = match current_monkey.arg2.as_str() {
            "old" => item,
            _ => monkeys[index].arg2.parse().unwrap()
        };
        let mut new_worry_level = match monkeys[index].op.as_str() {
            "*" => (arg1 * arg2),
            "+" => (arg1 + arg2),
            "-" => (arg1 - arg2),
            "/" => (arg1 / arg2),
            _ => unreachable!()
        };
        if new_worry_level > gcn {
            new_worry_level = new_worry_level % (gcn)
        }
        monkeys[index].number_of_inspections += 1;
        // Pass to new monkey
        match new_worry_level % (monkeys[index].test_divis as u128) == 0 {
            true => monkeys[current_monkey.true_endpoint as usize].items.push(new_worry_level),
            false => monkeys[current_monkey.false_endpoint as usize].items.push(new_worry_level),
        };
    }
    monkeys[index].items = Vec::new();
}


fn solve(monkeys:&mut Vec<Monkey>, rounds: u32, gcn: u128) {

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let_monkey_play(i, monkeys, gcn);
        }
    }
}

fn print_monkeys_items(monkeys: Vec<Monkey>) {
    for monkey in monkeys {
        println!("Monkey {}: {:?} inspections: {}", monkey.num, monkey.items, monkey.number_of_inspections);
    }
}

fn main() {
    let mut monkeys = read_file("input.txt");
    let gcn = monkeys.1;
    let mut monkeys = monkeys.0;
    solve(&mut monkeys, 10000, gcn);
    print_monkeys_items(monkeys);

}



#[test]
fn test_parse_op_args() {
    assert_eq!(("*", "1", "2"), Monkey::parse_op_args("Hello this is the world 1 * 2"))
}
