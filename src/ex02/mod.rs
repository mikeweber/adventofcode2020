use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    return Some(0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a() {
        if let Some(result) = crate::ex02::part_a(Some(&"./src/ex02/sample.txt".to_string())) {
            assert_eq!(result, 2);
        } else {
            panic!("Could not find a match");
        }
    }
}
