use std::fs;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    if let Some(path) = filename {
        let licenses = License::from_file(path);
        let valid_count = licenses.iter().fold(0, |count, license| {
            if license.is_valid() { count + 1 } else { count }
        });
        return Some(valid_count);
    }
    return None;
}

pub fn part_b(filename: Option<&String>) -> Option<u32> {
    if let Some(path) = filename {
        let licenses = License::from_file(path);
        let valid_count = licenses.iter().fold(0, |count, license| {
            if license.is_valid_v2() { count + 1 } else { count }
        });
        return Some(valid_count);
    }
    return None;
}

struct License {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl License {
    pub fn from_file(filename: &String) -> Vec<Self> {
        fs::read_to_string(filename).unwrap().split("\n\n").map(|lines| Self::new(&String::from(lines))).collect()
    }

    pub fn new(lines: &String) -> Self{
        let mut license = License::new_blank();
        for line in lines.split("\n") {
            for pair_str in line.split(" ") {
                let pair = pair_str.split(":").collect::<Vec<_>>();
                match pair[0] {
                    "byr" => { license.byr = Some(String::from(pair[1])) },
                    "iyr" => { license.iyr = Some(String::from(pair[1])) },
                    "eyr" => { license.eyr = Some(String::from(pair[1])) },
                    "hgt" => { license.hgt = Some(String::from(pair[1])) },
                    "hcl" => { license.hcl = Some(String::from(pair[1])) },
                    "ecl" => { license.ecl = Some(String::from(pair[1])) },
                    "pid" => { license.pid = Some(String::from(pair[1])) },
                    "cid" => { license.cid = Some(String::from(pair[1])) },
                    _     => {},
                }
            }
        }
        license
    }

    fn new_blank() -> Self {
        Self{
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }

    pub fn is_valid_v2(&self) -> bool {
        self.is_valid_byr() &&
            self.is_valid_iyr() &&
            self.is_valid_eyr() &&
            self.is_valid_hgt() &&
            self.is_valid_hcl() &&
            self.is_valid_ecl() &&
            self.is_valid_pid()
    }

    fn is_valid_byr(&self) -> bool {
        match &self.byr {
            Some(byr) => {
                match byr.parse::<u32>() {
                    Ok(byr_i) => { 1920 <= byr_i && byr_i <= 2002 },
                    Err(_e) => { false }
                }
            },
            None => { false }
        }
    }

    fn is_valid_iyr(&self) -> bool {
        match &self.iyr {
            Some(iyr) => {
                match iyr.parse::<u32>() {
                    Ok(iyr_i) => { 2010 <= iyr_i && iyr_i <= 2020 },
                    Err(_e) => { false }
                }
            },
            None => { false }
        }
    }

    fn is_valid_eyr(&self) -> bool {
        match &self.eyr {
            Some(eyr) => {
                match eyr.parse::<u32>() {
                    Ok(eyr_i) => { 2020 <= eyr_i && eyr_i <= 2030 },
                    Err(_e) => { false }
                }
            },
            None => { false }
        }
    }

    fn is_valid_hgt(&self) -> bool {
        match &self.hgt {
            Some(hgt) => {
                match hgt.get((hgt.len() - 2)..) {
                    Some(unit) => Self::is_valid_hgt_with_unit(hgt, unit.to_string()),
                    None => false,
                }
            },
            None => false,
        }
    }

    fn is_valid_hgt_with_unit(hgt: &String, unit: String) -> bool {
        let hgt_i = match hgt.get(..(hgt.len() - 2)).unwrap().parse::<u32>() {
            Ok(hgt_i) => hgt_i,
            Err(_e) => 0,
        };
        match unit.as_str() {
            "cm" => { 150 <= hgt_i && hgt_i <= 193 },
            "in" => { 59 <= hgt_i && hgt_i <= 76 },
            _ => false
        }
    }

    fn is_valid_hcl(&self) -> bool {
        let hcl = match &self.hcl {
            Some(hcl) => hcl,
            None => { return false; }
        };
        let car = match hcl.get(0..1) {
            Some(car) =>  car,
            None => { return false; }
        };
        if car != "#" { return false; }

        match hcl.get(1..) {
            Some(cdr) => cdr.len() == 6 && cdr.chars().filter(|ch| ch.is_digit(16)).collect::<Vec<char>>().len() == 6,
            None => false
        }
    }

    fn is_valid_ecl(&self) -> bool {
        match &self.ecl {
          Some(ecl) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str()),
          None => false
        }
    }

    fn is_valid_pid(&self) -> bool {
        match &self.pid {
            Some(pid) => pid.len() == 9 && pid.chars().fold(0, |count, ch| if ch.is_digit(10) { count + 1} else { count }) == 9,
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ex04::*;

    #[test]
    fn test_license_parser() {
        let license1 = License::new(&String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"));

        assert_eq!(license1.ecl.unwrap(), String::from("gry"));
        assert_eq!(license1.pid.unwrap(), String::from("860033327"));
        assert_eq!(license1.eyr.unwrap(), String::from("2020"));
        assert_eq!(license1.hcl.unwrap(), String::from("#fffffd"));
        assert_eq!(license1.byr.unwrap(), String::from("1937"));
        assert_eq!(license1.iyr.unwrap(), String::from("2017"));
        assert_eq!(license1.cid.unwrap(), String::from("147"));
        assert_eq!(license1.hgt.unwrap(), String::from("183cm"));
    }

    #[test]
    fn test_license_valid() {
        let license1 = License::new(&String::from("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm"));
        assert!(license1.is_valid());

        let license2 = License::new(&String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"));
        assert!(!license2.is_valid());
    }

    #[test]
    fn test_is_valid_byr() {
        let mut license = License::new_blank();
        license.byr = Some("2002".to_string());
        assert!(license.is_valid_byr());
        license.byr = Some("2003".to_string());
        assert!(!license.is_valid_byr());
        license.byr = Some("1919".to_string());
        assert!(!license.is_valid_byr());
        license.byr = Some("1920".to_string());
        assert!(license.is_valid_byr());
        license.byr = Some("foo".to_string());
        assert!(!license.is_valid_byr());
    }

    #[test]
    fn test_is_valid_iyr() {
        let mut license = License::new_blank();
        license.iyr = Some("2020".to_string());
        assert!(license.is_valid_iyr());
        license.iyr = Some("2021".to_string());
        assert!(!license.is_valid_iyr());
        license.iyr = Some("2009".to_string());
        assert!(!license.is_valid_iyr());
        license.iyr = Some("2010".to_string());
        assert!(license.is_valid_iyr());
        license.iyr = Some("foo".to_string());
        assert!(!license.is_valid_iyr());
    }

    #[test]
    fn test_is_valid_eyr() {
        let mut license = License::new_blank();
        license.eyr = Some("2030".to_string());
        assert!(license.is_valid_eyr());
        license.eyr = Some("2031".to_string());
        assert!(!license.is_valid_eyr());
        license.eyr = Some("2019".to_string());
        assert!(!license.is_valid_eyr());
        license.eyr = Some("2020".to_string());
        assert!(license.is_valid_eyr());
        license.eyr = Some("foo".to_string());
        assert!(!license.is_valid_eyr());
    }

    #[test]
    fn test_is_valid_hgt_cm() {
        let mut license = License::new_blank();
        license.hgt = Some("193cm".to_string());
        assert!(license.is_valid_hgt());
        license.hgt = Some("194cm".to_string());
        assert!(!license.is_valid_hgt());
        license.hgt = Some("149cm".to_string());
        assert!(!license.is_valid_hgt());
        license.hgt = Some("150cm".to_string());
        assert!(license.is_valid_hgt());
        license.hgt = Some("foo".to_string());
        assert!(!license.is_valid_hgt());
    }

    #[test]
    fn test_is_valid_hgt_in() {
        let mut license = License::new_blank();
        license.hgt = Some("76in".to_string());
        assert!(license.is_valid_hgt());
        license.hgt = Some("77in".to_string());
        assert!(!license.is_valid_hgt());
        license.hgt = Some("58in".to_string());
        assert!(!license.is_valid_hgt());
        license.hgt = Some("59in".to_string());
        assert!(license.is_valid_hgt());
        license.hgt = Some("thin".to_string());
        assert!(!license.is_valid_hgt());
    }

    #[test]
    fn test_is_valid_hcl() {
        let mut license = License::new_blank();
        license.hcl = Some("#123456".to_string());
        assert!(license.is_valid_hcl());
        license.hcl = Some("#9ab87c".to_string());
        assert!(license.is_valid_hcl());
        license.hcl = Some("123456".to_string());
        assert!(!license.is_valid_hcl());
        license.hcl = Some("#9abcd".to_string());
        assert!(!license.is_valid_hcl());
        license.hcl = Some("#9abcdez".to_string());
        assert!(!license.is_valid_hcl());
    }

    #[test]
    fn test_is_valid_ecl() {
        // ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        let mut license = License::new_blank();
        license.ecl = Some("amb".to_string());
        assert!(license.is_valid_ecl());
        license.ecl = Some("brn".to_string());
        assert!(license.is_valid_ecl());
        license.ecl = Some("brown".to_string());
        assert!(!license.is_valid_ecl());
        license.ecl = Some("".to_string());
        assert!(!license.is_valid_ecl());
    }

    #[test]
    fn test_is_valid_pid() {
        let mut license = License::new_blank();
        license.pid = Some("000000000".to_string());
        assert!(license.is_valid_pid());
        license.pid = Some("012345678".to_string());
        assert!(license.is_valid_pid());
        license.pid = Some("00000001".to_string());
        assert!(!license.is_valid_pid());
        license.pid = Some("0000000100".to_string());
        assert!(!license.is_valid_pid());
        license.pid = Some("0123a45678".to_string());
        assert!(!license.is_valid_pid());
    }
}
