use nom::{
    character::complete::{alpha1, newline},
    character::complete,
    bytes::complete::{tag, take_until},
    multi::separated_list1,
    sequence::delimited,
    branch::alt,
    IResult,
};


#[derive(Debug)]
pub struct Action {
    count: u32,
    from: u32,
    to: u32,
}

pub fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((tag("   "), delimited(tag("["), alpha1, tag("]"))))(input)?;
    let result = match c {
        "   " => None,
        val => Some(val),
    };
    Ok((input, result))
}

pub fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), parse_crate)(input)
}

pub fn parse_crate_lines(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>> {
    let (input, crates) = separated_list1(newline, parse_crate_line)(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    Ok((input, crates))
}

pub fn parse_action_line(input: &str) -> IResult<&str, Action> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Action {
            count: count,
            from: from - 1,
            to: to - 1,
        },
    ))
}

pub fn parse_action_lines(input: &str) -> IResult<&str, Vec<Action>> {
    separated_list1(newline, parse_action_line)(input)
}

pub fn process_part1(input: &str) -> String {
    let (input, crate_lines) = parse_crate_lines(input).unwrap();
    let mut stacks: Vec<Vec<&str>> = vec![vec![]; crate_lines[0].len()];
    for line in crate_lines.iter().rev() {
        for (i, val) in line.iter().enumerate() {
            if let Some(v) = val {
                stacks[i].push(v);
            }
        }
    }

    let (_, actions) = parse_action_lines(input).unwrap();

    for action in actions.iter() {
        for _ in 0..action.count {
            let to_move = stacks[action.from as usize].pop().unwrap();
            stacks[action.to as usize].push(to_move);
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|val| *val)
        .collect::<Vec<_>>()
        .join("")
}

pub fn process_part2(input: &str) -> String {
    let (input, crate_lines) = parse_crate_lines(input).unwrap();
    let mut stacks: Vec<Vec<&str>> = vec![vec![]; crate_lines[0].len()];
    for line in crate_lines.iter().rev() {
        for (i, val) in line.iter().enumerate() {
            if let Some(v) = val {
                stacks[i].push(v);
            }
        }
    }

    let (_, actions) = parse_action_lines(input).unwrap();

    for action in actions.iter() {
        let mut tmp = vec![];
        for _ in 0..action.count {
            tmp.push(stacks[action.from as usize].pop().unwrap());
        }
        tmp.reverse();
        stacks[action.to as usize].extend(tmp);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|val| *val)
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
