use std::{cmp, iter};

fn lower_trees(
    line: (impl Iterator<Item = usize>, impl Iterator<Item = usize>),
    trees: &Vec<Vec<u32>>,
    tree: u32,
) -> bool {
    line.0
        .zip(line.1)
        .filter(|(i, j)| trees[*i][*j] >= tree)
        .count()
        == 0
}

fn lower_tree_count(
    line: (impl Iterator<Item = usize>, impl Iterator<Item = usize>),
    trees: &Vec<Vec<u32>>,
    tree: u32,
) -> u32 {
    let mut found = false;
    line.0
        .zip(line.1)
        .filter(|(i, j)| {
            if found {
                return false;
            }
            if trees[*i][*j] >= tree {
                found = true;
            }
            return true;
        })
        .count() as u32
}

pub fn process_part1(input: &str) -> String {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (m, n) = (trees.len(), trees[0].len());
    let mut visible: Vec<Vec<bool>> = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                visible[i][j] = true;
                continue;
            }

            let up = ((0..i).rev(), iter::repeat(j));
            let down = (i + 1..m, iter::repeat(j));
            let right = (iter::repeat(i), j + 1..n);
            let left = (iter::repeat(i), (0..j).rev());
            if lower_trees(up, &trees, trees[i][j])
                || lower_trees(down, &trees, trees[i][j])
                || lower_trees(right, &trees, trees[i][j])
                || lower_trees(left, &trees, trees[i][j])
            {
                visible[i][j] = true;
            }
        }
    }
    visible
        .into_iter()
        .flatten()
        .filter(|b| *b)
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (m, n) = (trees.len(), trees[0].len());

    let mut max_factor = 0;
    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                continue;
            }

            let up = ((0..i).rev(), iter::repeat(j));
            let down = (i + 1..m, iter::repeat(j));
            let right = (iter::repeat(i), j + 1..n);
            let left = (iter::repeat(i), (0..j).rev());
            let factor = lower_tree_count(up, &trees, trees[i][j])
                * lower_tree_count(down, &trees, trees[i][j])
                * lower_tree_count(right, &trees, trees[i][j])
                * lower_tree_count(left, &trees, trees[i][j]);
            max_factor = cmp::max(max_factor, factor);
        }
    }
    max_factor.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "8");
    }
}
