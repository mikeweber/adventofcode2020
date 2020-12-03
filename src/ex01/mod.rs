use crate::utils::*;

const TARGET_VALUE: usize = 2020;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    let report_values = match filename {
        Some(path) => parse_report(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };
    return find_value(TARGET_VALUE, &report_values);
}

pub fn part_b(filename: Option<&String>) -> Option<u32> {
    let report_values = match filename {
        Some(path) => parse_report(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };
    return find_triple_value(TARGET_VALUE, &report_values);
}

fn parse_report(filename: &String) -> [bool; TARGET_VALUE] {
    let mut expense_reports: [bool; TARGET_VALUE] = [false; TARGET_VALUE];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(r) = line {
                let val = r.parse::<usize>().unwrap();
                expense_reports[val] = true;
            }
        }
    }

    return expense_reports;
}

fn find_triple_value(target_value: usize, report_values: &[bool; TARGET_VALUE]) -> Option<u32> {
    for (i, in_list1) in report_values.iter().enumerate() {
        if *in_list1 {
            for (j, in_list2) in report_values[(i + 1)..report_values.len()].iter().enumerate() {
                if *in_list2 {
                    // We're looping through a sublist, so the enumerator is going to
                    // start over at an index of 0. Add back the fqct that we're starting
                    // at an index of `i + 1`.
                    let val2 = j + i + 1;
                    let diff = target_value - (i + val2);
                    if report_values[diff] {
                        return Some((i * val2 * diff) as u32);
                    }
                }
            }
        }
    }
    return None;
}

fn find_value(target_value: usize, report_values: &[bool; TARGET_VALUE]) -> Option<u32> {
    for (i, in_list) in report_values.iter().enumerate() {
        if *in_list {
            let diff = target_value - i;
            if report_values[diff] {
                return Some((i * diff) as u32);
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a() {
        if let Some(result) = crate::ex01::part_a(Some(&"./src/ex01/sample.txt".to_string())) {
            assert_eq!(result, 514579);
        } else {
            panic!("Could not find a match");
        }
    }

    #[test]
    fn test_part_b() {
        if let Some(result) = crate::ex01::part_b(Some(&"./src/ex01/sample.txt".to_string())) {
            assert_eq!(result, 241861950);
        } else {
            panic!("Could not find a match");
        }
    }
}

