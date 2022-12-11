use nom::{
    branch::alt, bytes::complete::tag, character::complete, multi::separated_list1,
    sequence::preceded, *,
};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Noop,
    Add(i32),
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, mut operations) = separated_list1(
        complete::newline,
        alt((
            tag("noop").map(|_| Operation::Noop),
            preceded(tag("addx "), complete::i32).map(|add| Operation::Add(add)),
        )),
    )(input)?;
    operations = operations
        .iter()
        .flat_map(|op| match op {
            Operation::Noop => vec![Operation::Noop],
            Operation::Add(add) => vec![Operation::Noop, Operation::Add(*add)],
        })
        .collect();
    Ok((input, operations))
}

pub fn process_part1(input: &str) -> String {
    let (_, operations) = parse_operations(input).unwrap();

    let mut x = 1;
    let mut i = 1;
    let mut total = 0;
    for operation in operations {
        if (i - 20) % 40 == 0 {
            total += x * i;
        }
        if let Operation::Add(add) = operation {
            x += add;
        }
        i += 1;
    }

    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, operations) = parse_operations(input).unwrap();

    let mut x = 1;
    let mut i = 0;
    let mut crt: Vec<Vec<char>> = vec![vec![]];
    for operation in operations {
        let mut n = crt.len() - 1;
        if crt[n].len() == 40 {
            crt.push(vec![]);
            n += 1;
        }
        if (x - 1..=x + 1).contains(&(i % 40)) {
            crt[n].push('#');
        } else {
            crt[n].push('.');
        }
        if let Operation::Add(add) = operation {
            x += add;
        }
        i += 1;
    }

    crt.into_iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
