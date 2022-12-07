use std::collections::{VecDeque, HashSet};


pub fn process_part1(input: &str) -> String {
    let mut deq: VecDeque<char>  = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        if deq.len() == 4 {
            let hash_set: HashSet<_> = deq.clone().into_iter().collect();
            if hash_set.len() == 4 {
                return i.to_string()
            }
            deq.pop_front().unwrap();
        }
        deq.push_back(c);
    }
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut deq: VecDeque<char>  = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        if deq.len() == 14 {
            let hash_set: HashSet<_> = deq.clone().into_iter().collect();
            if hash_set.len() == 14 {
                return i.to_string()
            }
            deq.pop_front().unwrap();
        }
        deq.push_back(c);
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "11");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "26");
    }
}
