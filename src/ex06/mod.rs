use std::fs;
use std::collections::hash_map::*;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    match filename {
        Some(path) => Some(sum_groups(parse_groups(fs::read_to_string(path).unwrap()))),
        None => None
    }
}

pub fn part_b(filename: Option<&String>) -> Option<u32> {
    match filename {
        Some(path) => Some(sum_groups(parse_groups_in_agreement(fs::read_to_string(path).unwrap()))),
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

    for ch in group_str.chars() {
        if ch == '\n' {
            if new_line {
                groups.push(cur_group.len() as u32);
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
            }
        }
    }
    if !cur_group.is_empty() {
        groups.push(cur_group.len() as u32);
    }
    return groups;
}

fn parse_groups_in_agreement(group_str: String) -> Vec<u32> {
    let mut new_line = false;
    let mut groups = vec![];
    let mut cur_group: HashMap<char, u32> = HashMap::new();
    let mut group_size = 0;

    for ch in group_str.chars() {
        if ch == '\n' {
            if new_line {
                groups.push(count_agreements(cur_group.drain(), group_size));
                new_line = false;
                group_size = 0;
            } else {
                group_size += 1;
                new_line = true;
            }
        } else {
            new_line = false;
            let answer_size = match cur_group.get(&ch) {
                Some(count) => count + 1,
                None => 1,
            };
            cur_group.insert(ch, answer_size);
        }
    }
    if !cur_group.is_empty() {
        groups.push(count_agreements(cur_group.drain(), group_size));
    }
    return groups;
}

fn count_agreements(group: Drain<char, u32>, group_size: u32) -> u32 {
    group.fold(0, |count, (ch, answer_size)| {
        if answer_size == group_size { count + 1 } else { count }
    })
}

#[cfg(test)]
mod tests {
    use crate::ex06::*;

    #[test]
    fn test_parsing_groups() {
        let grp_str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\n\nb".to_string();
        assert_eq!(parse_groups(grp_str), vec![3, 3, 3, 1, 1]);
    }

    #[test]
    fn test_parsing_groups_in_agreement() {
        let grp_str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\n\nb".to_string();
        assert_eq!(parse_groups_in_agreement(grp_str.clone()), vec![3, 0, 1, 1, 1]);
        assert_eq!(sum_groups(parse_groups_in_agreement(grp_str.clone())), 6);
    }
}
