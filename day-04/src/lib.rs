use std::cmp;

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (l, r) = line.split_once(",").unwrap();
            let (ls, le) = l.split_once("-").unwrap();
            let (rs, re) = r.split_once("-").unwrap();

            let (nls, nle, nrs, nre): (u32, u32, u32, u32) = (
                ls.parse().unwrap(),
                le.parse().unwrap(),
                rs.parse().unwrap(),
                re.parse().unwrap(),
            );
                
            if (nls <= nrs && nle >= nre) || (nrs <= nls && nre >= nle) {
                return Some(1)
            }
            None
        })
        .collect::<Vec<_>>()
        .len()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (l, r) = line.split_once(",").unwrap();
            let (ls, le) = l.split_once("-").unwrap();
            let (rs, re) = r.split_once("-").unwrap();
            let (nls, nle, nrs, nre): (u32, u32, u32, u32) = (
                ls.parse().unwrap(),
                le.parse().unwrap(),
                rs.parse().unwrap(),
                re.parse().unwrap(),
            );

            if cmp::max(nls, nrs) <= cmp::min(nle, nre) {
                return Some(1)
            }
            None
        })
        .collect::<Vec<_>>()
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
