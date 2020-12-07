use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    None
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
        } else if !cur_group.contains(&ch) {
            cur_group.push(ch);
            new_line = false;
        }
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
