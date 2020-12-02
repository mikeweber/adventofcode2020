use std::collections::HashMap;
use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> u32 {
    let report_values = match filename {
        Some(path) => parse_report(path),
        None => {
            println!("Please specify which file you'd like to run");
            return 0;
        }
    };
    return find_value(2020, &report_values);
}

fn parse_report(filename: &String) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(r) = line {
                let val = r.parse::<u32>().unwrap();
                map.insert(val, val);
            }
        }
    }

    return map;
}

fn find_value(target_value: u32, report_values: &HashMap<u32, u32>) -> u32 {
    for (value, _) in report_values {
        let diff = target_value - value;
        if let Some(other_value) = report_values.get(&diff) {
            return other_value * value;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn with_sample() {
        let result = crate::ex1::part_a(Some(&"./src/ex1/sample.txt".to_string()));
        assert_eq!(result, 514579);
    }
}

