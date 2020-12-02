use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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
        println!("Opened file");
        for line in lines {
            if let Ok(r) = line {
                println!("Unwrapping value...");
                let val = r.parse::<u32>().unwrap();
                println!("Unwrapped {}", val);
                map.insert(val, val);
            }
        }
    }

    return map;
}

fn find_value(target_value: u32, report_values: &HashMap<u32, u32>) -> u32 {
    println!("Finding a value {}", target_value);
    for (value, _) in report_values {
        let diff = target_value - value;
        println!("Testing {} and {}", value, diff);
        if let Some(other_value) = report_values.get(&diff) {
            println!("Match!");
            return other_value * value;
        }
    }

    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    #[test]
    fn with_sample() {
        let result = crate::ex1::part_a(Some(&"./src/ex1/sample.txt".to_string()));
        assert_eq!(result, 514579);
    }
}

