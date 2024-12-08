use std::fs;
use anyhow::Result;

pub fn solve() {
    let lists = read_input("inputs/day02.txt").unwrap();
    let monotonous: Vec<_> = lists.iter()
        .filter(|list| is_monotonous(list))
        .collect();
    let safe_removable: Vec<_> = lists.iter()
        .filter(|list| safe_by_removal(list))
        .collect();
    
    println!("Part 1 (monotonous): {}", monotonous.len());
    println!("Part 2 (safe removable): {}", safe_removable.len());
}

pub fn read_input(path: &str) -> Result<Vec<Vec<i32>>> {
    let input = fs::read_to_string(path)?;
    let list: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    Ok(list)
}

pub fn is_monotonous(list: &Vec<i32>) -> bool {
    if list.len() <= 1 {
        return true;
    }
    let increasing = list.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff > 0 && diff <= 3
    });
    
    let decreasing = list.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff > 0 && diff <= 3
    });
    
    increasing || decreasing
}

pub fn safe_by_removal(list: &Vec<i32>) -> bool {
    if list.len() <= 2 {
        return true;
    }
    
    for i in 0..list.len() {
        let mut new_list: Vec<i32> = list.clone();
        new_list.remove(i);
        if is_monotonous(&new_list) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_monotonous() {
        assert!(is_monotonous(&vec![1, 2, 3]));     // increasing by 1
        assert!(is_monotonous(&vec![1, 3, 5]));     // increasing by 2
        assert!(is_monotonous(&vec![1, 4, 7]));     // increasing by 3
        assert!(is_monotonous(&vec![7, 4, 1]));     // decreasing by 3
        assert!(is_monotonous(&vec![5, 3, 1]));     // decreasing by 2
        assert!(is_monotonous(&vec![3, 2, 1]));     // decreasing by 1
        assert!(is_monotonous(&vec![1]));           // single element
        assert!(is_monotonous(&vec![]));            // empty
        assert!(!is_monotonous(&vec![1, 5, 7]));    // first gap too large
        assert!(!is_monotonous(&vec![1, 3, 2]));    // not monotonous
        assert!(!is_monotonous(&vec![3, 1, 2]));    // not monotonous
    }

    #[test]
    fn test_safe_by_removal() {
        assert!(safe_by_removal(&vec![1, 2, 3]));
        assert!(safe_by_removal(&vec![3, 2, 1]));
        assert!(safe_by_removal(&vec![1, 2, 4]));
        assert!(safe_by_removal(&vec![1, 5, 7]));
        assert!(safe_by_removal(&vec![1, 3, 2]));
        assert!(safe_by_removal(&vec![3, 1, 2]));
    }
 
}
