use std::fs;
use anyhow::Result;
use regex::Regex;

pub fn solve() {
    part1();
    part2();
}

pub fn part2() {
    let indices = read_input_part2("inputs/day03.txt").unwrap();
    let input = fs::read_to_string("inputs/day03.txt").unwrap();
    let mut all_multiplications = Vec::new();
    
    for (start, end) in indices {
        let slice_multiplications = parse_multiplications(&input[start as usize..end as usize]);
        all_multiplications.extend(slice_multiplications);
    }
    
    let result = all_multiplications.iter()
        .map(|(x, y)| x * y)
        .sum::<i32>();
    println!("Part 2: {}", result);
}

pub fn read_input_part2(path: &str) -> Result<Vec<(i32, i32)>> {
    let input = fs::read_to_string(path)?;
    Ok(find_string_indices(&input))
}

fn find_string_indices(input: &str) -> Vec<(i32, i32)> {
    let mut indices = Vec::new();
    indices.push((0, 0));
    for (idx, _) in input.match_indices("do()") {
        indices.push((idx as i32, 0));
    }
    
    for (idx, _) in input.match_indices("don't()") {
        indices.push((idx as i32, 1));
    }
    
    indices.sort_by_key(|&(idx, _)| idx);
    
    let mut paired_indices = Vec::new();
    let mut last_do_idx = None;
    
    for (idx, type_) in indices {
        if type_ == 0 {  // do()
            if last_do_idx.is_none() {
                last_do_idx = Some(idx);
            }
        } else {  // don't()
            if let Some(do_idx) = last_do_idx {
                paired_indices.push((do_idx, idx));
                last_do_idx = None;
            }
        }
    }
    
    paired_indices
}

pub fn part1() {
    let multiplications: Vec<(i32, i32)> = read_input_part1("inputs/day03.txt").unwrap();
    let result = multiplications.iter()
        .map(|(x, y)| x * y)
        .sum::<i32>();
    println!("Part 1: {}", result);
}
pub fn read_input_part1(path: &str) -> Result<Vec<(i32, i32)>> {
    let input = fs::read_to_string(path)?;
    Ok(parse_multiplications(&input))
}

fn parse_multiplications(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let x = caps[1].parse::<i32>().unwrap();
            let y = caps[2].parse::<i32>().unwrap();
            (x, y)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiplications() {
        let input = "here is mul(123,456) and another mul(789,321) and mul(1,2)";
        let result = parse_multiplications(input);
        assert_eq!(result, vec![(123, 456), (789, 321), (1, 2)]);
    }
}