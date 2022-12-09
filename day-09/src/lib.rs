use std::{cmp, collections::HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Right,
    Left,
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, dir) = alt((
        complete::char('L').map(|_| Move::Left),
        complete::char('R').map(|_| Move::Right),
        complete::char('U').map(|_| Move::Up),
        complete::char('D').map(|_| Move::Down),
    ))(input)?;
    Ok((input, dir))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, parsed_lines) = separated_list1(
        complete::newline,
        separated_pair(parse_move, tag(" "), complete::u32),
    )(input)?;

    let moves = parsed_lines
        .iter()
        .flat_map(|(mv, repeat)| vec![*mv; *repeat as usize])
        .collect();
    Ok((input, moves))
}

pub fn process_part1(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited = HashSet::from([tail]);
    for head_move in moves.iter() {
        let saved_head = head.clone();
        match head_move {
            Move::Up => head.0 -= 1,
            Move::Down => head.0 += 1,
            Move::Right => head.1 += 1,
            Move::Left => head.1 -= 1,
        };
        let distance = cmp::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs());
        if distance > 1 {
            tail = saved_head;
            visited.insert(tail);
        }
    }

    visited.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut visited = HashSet::from([(0, 0)]);
    for head_move in moves.iter() {
        match head_move {
            Move::Up => rope[0].0 -= 1,
            Move::Down => rope[0].0 += 1,
            Move::Right => rope[0].1 += 1,
            Move::Left => rope[0].1 -= 1,
        };

        for i in 1..10 {
            let head = rope[i - 1];
            let tail = rope[i];
            let distance = cmp::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs());
            if distance > 1 {
                rope[i].0 += (head.0 - tail.0).signum();
                rope[i].1 += (head.1 - tail.1).signum();
            }
        }
        visited.insert(rope[rope.len() - 1]);
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "1");
        assert_eq!(process_part2(INPUT_2), "36")
    }
}
