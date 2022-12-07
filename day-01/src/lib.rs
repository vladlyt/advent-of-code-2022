pub fn process_part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|line| line.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap()
        .to_string()
}
pub fn process_part2(input: &str) -> String {
    let mut callories = input
        .split("\n\n")
        .map(|line| line.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    callories.sort_by(|a, b| b.cmp(a));
    callories.iter().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
