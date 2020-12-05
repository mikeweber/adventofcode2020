use std::fs;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    if let Some(path) = filename {
        let tree_map = TreeMap::from_file(path);
        return Some(tree_map.encounters(3, 1));
    }
    return None;
}

pub fn part_b(filename: Option<&String>) -> Option<u32> {
    if let Some(path) = filename {
        let tree_map = TreeMap::from_file(path);
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let product = slopes.iter().fold(1, |product, (rise, run)| {
            product * tree_map.encounters(*rise, *run)
        });
        return Some(product);
    }
    return None;
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
enum Marker {
    Open,
    Tree
}

struct TreeMap {
    rows:  Vec<Vec<Marker>>,
}

impl TreeMap {
    pub fn from_file(filename: &String) -> Self {
        Self::new(&fs::read_to_string(filename).unwrap())
    }

    pub fn new(lines: &String) -> Self {
        let mut rows = vec![];
        for line in lines.split("\n").filter(|l| l.len() > 0) {
            let mut row = vec![];
            for ch in line.chars() {
                match ch {
                    '.' => row.push(Marker::Open),
                    '#' => row.push(Marker::Tree),
                    _   => row.push(Marker::Open),
                }
            }
            rows.push(row);
        }

        Self{ rows: rows }
    }

    pub fn encounters(&self, run_len: usize, rise: usize) -> u32 {
        self.rows.iter().enumerate().fold(0, |count, (row_index, row)| {
            if row_index % rise == 0 {
                match row[(row_index / rise * run_len) % row.len()] {
                    Marker::Open => count,
                    Marker::Tree => count + 1,
                }
            } else {
                count
            }
        })
    }

    pub fn get(&self, row_index: usize, col_index: usize) -> Marker {
        let row = &self.rows[row_index];
        let width = row.len();
        let limited_col_index = col_index % width;
        row[limited_col_index]
    }
}

#[cfg(test)]
mod tests {
    use crate::ex03::*;

    #[test]
    fn test_map_parser() {
        let sample_map: String = String::from("..#\n\
#..\n\
.##");
        let tree_map = TreeMap::new(&sample_map);
        assert_eq!(tree_map.get(0, 0), Marker::Open);
        assert_eq!(tree_map.get(0, 1), Marker::Open);
        assert_eq!(tree_map.get(0, 2), Marker::Tree);
        assert_eq!(tree_map.get(0, 3), Marker::Open);
        assert_eq!(tree_map.get(0, 5), Marker::Tree);

        assert_eq!(tree_map.get(1, 0), Marker::Tree);
        assert_eq!(tree_map.get(1, 1), Marker::Open);
        assert_eq!(tree_map.get(1, 2), Marker::Open);
        assert_eq!(tree_map.get(1, 3), Marker::Tree);

        assert_eq!(tree_map.get(2, 0), Marker::Open);
        assert_eq!(tree_map.get(2, 1), Marker::Tree);
        assert_eq!(tree_map.get(2, 2), Marker::Tree);
        assert_eq!(tree_map.get(2, 3), Marker::Open);
        assert_eq!(tree_map.get(2, 4), Marker::Tree);
    }

    #[test]
    fn test_tree_encounters() {
        let tree_map = TreeMap::from_file(&"./src/ex03/sample.txt".to_string());
        assert_eq!(tree_map.encounters(1, 1), 2);
        assert_eq!(tree_map.encounters(3, 1), 7);
        assert_eq!(tree_map.encounters(5, 1), 3);
        assert_eq!(tree_map.encounters(7, 1), 4);
        assert_eq!(tree_map.encounters(1, 2), 2);
    }
}
