use std::fs;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    match filename {
        Some(path) => Some(sum_groups(parse_groups(fs::read_to_string(path).unwrap()))),
        None => None
    }
}

fn sum_groups(groups: Vec<u32>) -> u32 {
    groups.iter().fold(0, |count, sum| count + sum)
}

fn parse_groups(group_str: String) -> Vec<u32> {
    let mut new_line = false;
    let mut groups = vec![];
    let mut cur_group = vec![];
    let mut sum = 0;

    for ch in group_str.chars() {
        if ch == '\n' {
            if new_line {
                groups.push(cur_group.len() as u32);
                sum += cur_group.len() as u32;
                println!("cur_group ({}/{}): {:?}", cur_group.len(), sum, cur_group);
                cur_group.clear();
                new_line = false;
            } else {
                new_line = true;
            }
        } else {
            new_line = false;
            if !cur_group.contains(&ch) {
                cur_group.push(ch);
                cur_group.sort();
                new_line = false;
            }
        }
    }
    if !cur_group.is_empty() {
        groups.push(cur_group.len() as u32);
    }
    return groups;
}

#[cfg(test)]
mod tests {
    use crate::ex06::*;

    #[test]
    fn test_parsing_groups() {
        let grp_str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\n\nb".to_string();
        assert_eq!(parse_groups(grp_str), vec![3, 3, 3, 1, 1]);
    }
}
