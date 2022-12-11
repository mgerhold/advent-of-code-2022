use std::fmt::Display;

enum Token {
    Integer(u64),
    Plus,
    Asterisk,
    New,
    Old,
    Equals,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Token::Integer(value) = self {
            f.write_fmt(format_args!("{value}"))
        } else {
            f.write_fmt(format_args!(
                "{}",
                match self {
                    Token::Plus => "+",
                    Token::Asterisk => "*",
                    Token::New => "new",
                    Token::Old => "old",
                    Token::Equals => "=",
                    _ => unreachable!(),
                }
            ))
        }
    }
}

struct Operation {
    tokens: Vec<Token>,
}

impl Operation {
    pub fn from_string(string: &str) -> Self {
        let parts = string.split(' ');
        let tokens: Vec<_> = parts
            .map(str::trim)
            .map(|s| match s {
                "+" => Token::Plus,
                "*" => Token::Asterisk,
                "=" => Token::Equals,
                "new" => Token::New,
                "old" => Token::Old,
                _ => Token::Integer(s.parse().unwrap()),
            })
            .collect();
        Self { tokens }
    }

    pub fn evaluate(&self, old_value: u64) -> u64 {
        assert!(matches!(self.tokens[0], Token::New));
        assert!(matches!(self.tokens[1], Token::Equals));
        assert!(matches!(self.tokens[2], Token::Old));
        assert!(matches!(self.tokens[3], Token::Plus) || matches!(self.tokens[3], Token::Asterisk));
        let lhs = old_value;
        let rhs = match self.tokens[4] {
            Token::Integer(value) => value,
            Token::Old => old_value,
            _ => unreachable!(),
        };
        match self.tokens[3] {
            Token::Plus => lhs + rhs,
            Token::Asterisk => lhs * rhs,
            _ => unreachable!(),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tokens: Vec<_> = self.tokens.iter().map(|t| format!("{t}")).collect();
        f.write_fmt(format_args!("{}", tokens.join(" ")))
    }
}

struct Monkey {
    worry_levels: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    target_monkey_if_true: u64,
    target_monkey_if_false: u64,
    number_of_inspections: u64,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let worry_levels: Vec<_> = self.worry_levels.iter().map(|n| format!("{n}")).collect();
        f.write_fmt(format_args!(
            "Starting items: {}\nOperation: {}\nTest: divisible by {}\n  If true: throw to monkey {}\n  If false: throw to monkey {}\n",
            worry_levels.join(", "),
            self.operation,
            self.test_divisor,
            self.target_monkey_if_true,
            self.target_monkey_if_false,
        ))
    }
}

#[must_use]
fn parse_input(string: &str) -> Vec<Monkey> {
    let lines: Vec<_> = string.lines().collect();
    let mut monkeys = Vec::new();
    for i in (0..lines.len()).step_by(7) {
        let monkey = lines[i]
            .split(' ')
            .nth(1)
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<u64>()
            .unwrap();
        assert!(monkey == (i as u64) / 7);
        let worry_levels: Vec<_> = lines[i + 1]
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(str::trim)
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect();
        let operation = Operation::from_string(lines[i + 2].split(':').last().unwrap().trim());
        let get_trailing_number = |line_index: usize| {
            lines[line_index]
                .split(' ')
                .last()
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap()
        };
        let test_divisor = get_trailing_number(i + 3);
        let target_monkey_when_true = get_trailing_number(i + 4);
        let target_monkey_when_false = get_trailing_number(i + 5);
        monkeys.push(Monkey {
            worry_levels,
            operation,
            test_divisor,
            target_monkey_if_true: target_monkey_when_true,
            target_monkey_if_false: target_monkey_when_false,
            number_of_inspections: 0,
        });
    }
    monkeys
}

struct ThrowAction {
    target_monkey: usize,
    worry_level: u64,
}

fn prime_reduce(n: u64) -> u64 {
    const PRIMES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];
    let product: u64 = PRIMES.iter().product();

    n % product
    /*let mut is_divisible = [false; PRIMES.len()];
    for (i, prime) in PRIMES.iter().enumerate() {
        is_divisible[i] = n % prime == 0;
    }

    for prime in PRIMES {
        while n % (prime * prime) == 0 {
            n /= prime;
        }
    }
    n*/
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    Part1,
    Part2,
}

fn make_turn(monkeys: &mut [Monkey], monkey_index: usize, part: Part) {
    let monkey = &mut monkeys[monkey_index];
    let mut to_throw = Vec::with_capacity(monkey.worry_levels.len());
    for worry_level in &monkey.worry_levels {
        //println!("  Monkey inspects an item with a worry level of {worry_level}.");
        let new_worry_level = monkey.operation.evaluate(*worry_level);
        //println!("    New worry level is {new_worry_level}.");

        let new_worry_level = if part == Part::Part1 {
            new_worry_level / 3
        } else {
            prime_reduce(new_worry_level)
        };

        //println!("    Monkey gets bored => {new_worry_level}.");

        let target_monkey = if new_worry_level % monkey.test_divisor == 0 {
            monkey.target_monkey_if_true
        } else {
            monkey.target_monkey_if_false
        };
        /*println!(
            "    Item with worry level {new_worry_level} is thrown to monkey {target_monkey}."
        );*/
        to_throw.push(ThrowAction {
            target_monkey: target_monkey as usize,
            worry_level: new_worry_level,
        });
    }
    monkey.number_of_inspections += monkey.worry_levels.len() as u64;
    monkey.worry_levels.clear();
    for throw_action in to_throw {
        let ThrowAction {
            target_monkey,
            worry_level,
        } = throw_action;
        monkeys[target_monkey].worry_levels.push(worry_level);
    }
}

fn make_round(monkeys: &mut Vec<Monkey>, part: Part) {
    for i in 0..monkeys.len() {
        //println!("Monkey {i}:");
        make_turn(monkeys, i, part);
    }
}

fn main() {
    let contents = std::fs::read_to_string("real_input.txt").unwrap();
    let mut monkeys = parse_input(&contents);
    /*for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i}:\n{monkey}");
    }*/
    const NUM_ROUNDS: usize = 10000;

    for _ in 0..NUM_ROUNDS {
        make_round(&mut monkeys, Part::Part2);
    }
    for (i, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {i} inspected items {} times.",
            monkey.number_of_inspections
        );
    }

    let (max, second_to_max) = monkeys
        .iter()
        .map(|monkey| monkey.number_of_inspections)
        .fold((0, 0), |(max, second_to_max), x| {
            if x > max {
                (x, max)
            } else if x > second_to_max {
                (max, x)
            } else {
                (max, second_to_max)
            }
        });
    println!("max: {max}\nsecond to max: {second_to_max}");
    println!("result: {}", max * second_to_max);
}
