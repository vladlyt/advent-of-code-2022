#![feature(iter_next_chunk)]

use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left_set: HashSet<_> = left.chars().collect();
            let right_set: HashSet<_> = right.chars().collect();
            let common = *left_set.intersection(&right_set).collect::<Vec<_>>()[0];
            if common.is_uppercase() {
                common as u32 - 'A' as u32 + 27
            } else {
                common as u32 - 'a' as u32 + 1
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut lines = input.lines();

    let mut sum = 0;
    while let Ok([first, second, third]) = lines.next_chunk() {
        let first_set: HashSet<_> = first.chars().collect();
        let second_set: HashSet<_> = second.chars().collect();
        let third_set: HashSet<_> = third.chars().collect();

        let common = *first_set
            .intersection(&second_set)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&third_set)
            .collect::<Vec<&char>>()[0];

        sum += {
            if common.is_uppercase() {
                common as u32 - 'A' as u32 + 27
            } else {
                common as u32 - 'a' as u32 + 1
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
