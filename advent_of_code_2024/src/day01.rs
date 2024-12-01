use std::fs;
use anyhow::Result;
use std::collections::HashMap;

pub fn solve() {
    let (left, right) = read_input("inputs/day01.txt").unwrap();
    println!("Part 1: {}", compare_list_difference(&left, &right));
    println!("Part 2: {}", calculate_similarity_score(&left, &right));
}

pub fn compare_list_difference(l_list: &Vec<i32>, r_list: &Vec<i32>) -> i32 {
    let mut l_list = l_list.clone();
    let mut r_list = r_list.clone();
    l_list.sort();
    r_list.sort();

    let mut diff_sum = 0;
    while let (Some(l), Some(r)) = (l_list.pop(), r_list.pop()) {
        diff_sum += (l - r).abs();
    }
    return diff_sum;
}

pub fn read_input(path: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let input = fs::read_to_string(path)?;
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();
    Ok((left, right))
}

pub fn calculate_similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut counter_map = HashMap::new();
    for num in right {
        *counter_map.entry(num).or_insert(0) += 1;
    }

    let mut score = 0;
    for num in left {
        if !counter_map.contains_key(num) {
            continue;
        }
        score += num * counter_map.get(num).unwrap();
    }
    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_list_difference() {
        assert_eq!(compare_list_difference(&vec![1, 2, 3], &vec![3, 2, 1]), 0);
        assert_eq!(compare_list_difference(&vec![1, 2, 3], &vec![1, 2, 3]), 0);
        assert_eq!(compare_list_difference(&vec![1, 2, 3], &vec![4, 5, 6]), 9);
    }

    #[test]
    fn test_calculate_similarity_score() {
        assert_eq!(calculate_similarity_score(&vec![1, 2, 3], &vec![3, 2, 1]), 6);
    }
}
