use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    *,
};

#[derive(Debug)]
enum Operation {
    Add(i32),
    Multiply(i32),
    Square,
}

#[derive(Debug)]
struct MonkeyTest {
    divisible_by: i32,
    throw_true: usize,
    throw_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: MonkeyTest,
}

fn parse_test(input: &str) -> IResult<&str, MonkeyTest> {
    let (input, divisible_by) = preceded(tag("  Test: divisible by "), complete::i32)(input)?;
    let (input, _) = newline(input)?;
    let (input, throw_true) = preceded(tag("    If true: throw to monkey "), complete::u32)(input)?;
    let (input, _) = newline(input)?;
    let (input, throw_false) =
        preceded(tag("    If false: throw to monkey "), complete::u32)(input)?;
    Ok((
        input,
        MonkeyTest {
            divisible_by,
            throw_true: throw_true as usize,
            throw_false: throw_false as usize,
        },
    ))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        separated_pair(tag("+"), tag(" "), complete::i32).map(|(_, num)| Operation::Add(num)),
        separated_pair(tag("*"), tag(" "), complete::i32).map(|(_, num)| Operation::Multiply(num)),
        separated_pair(tag("*"), tag(" "), tag("old")).map(|(_, _)| Operation::Square),
    ))(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = newline(input)?;

    let (input, items) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), complete::i32),
    )(input)?;
    let (input, _) = newline(input)?;

    let (input, operation) = preceded(tag("  Operation: new = old "), parse_operation)(input)?;
    let (input, _) = newline(input)?;

    let (input, test) = parse_test(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), parse_monkey)(input)
}

impl Monkey {
    fn throw_items(&mut self, with_relief: bool) -> Vec<(i32, usize)> {
        let mut to_throw: Vec<(i32, usize)> = vec![];
        for item in &self.items {
            let mut new_item = match self.operation {
                Operation::Add(n) => item + n,
                Operation::Multiply(n) => item * n,
                Operation::Square => item * item,
            };
            if with_relief {
                new_item /= 3;
            }

            let new_monkey = match new_item % self.test.divisible_by == 0 {
                true => self.test.throw_true,
                false => self.test.throw_false,
            };
            to_throw.push((new_item, new_monkey));
        }
        self.items.clear();
        to_throw
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();

    let mut item_counts: Vec<u32> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            item_counts[i] += monkeys[i].items.len() as u32;
            for (item, to_monkey) in monkeys[i].throw_items(true) {
                monkeys[to_monkey].items.push(item);
            }
        }
    }

    item_counts.sort_by(|a, b| b.cmp(a));
    item_counts
        .iter()
        .take(2)
        .fold(1, |acc, e| acc * e)
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();

    let mut item_counts: Vec<u32> = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            item_counts[i] += monkeys[i].items.len() as u32;
            for (item, to_monkey) in monkeys[i].throw_items(false) {
                monkeys[to_monkey].items.push(item);
            }
        }
    }

    item_counts.sort_by(|a, b| b.cmp(a));
    item_counts
        .iter()
        .take(2)
        .fold(1, |acc, e| acc * e)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
