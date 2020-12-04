use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
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
    pub fn new(lines: &String) -> Self {
        let mut rows = vec![];
        for line in lines.split("\n") {
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

    pub fn get(&self, row_index: usize, col_index: usize) -> Marker {
        let row = &self.rows[row_index];
        let width = row.len();
        let limited_col_index = col_index % width;
        println!("{} % {} => {}", col_index, width, limited_col_index);
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
}
