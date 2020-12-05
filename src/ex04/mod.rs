use std::fs;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
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
}
