use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    if let Some(path) = filename {
        let passwords = parse_from_file(path);
        return Some(passwords.iter().fold(0, |count, x| if x.is_valid() { count + 1 } else { count } ));
    }

    return None;
}

pub fn part_b(filename: Option<&String>) -> Option<u32> {
    if let Some(path) = filename {
        let passwords = parse_from_file(path);
        return Some(passwords.iter().fold(0, |count, x| if x.is_valid_v2() { count + 1 } else { count } ));
    }

    return None;

}

fn parse_from_file(filename: &String) -> Vec<PasswordRule> {
    let mut rules = vec!();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(r) = line {
                rules.push(PasswordRule::new(&r));
            }
        }
    }

    return rules;
}

struct PasswordRule {
    min:    usize,
    max:    usize,
    letter: char,
    pass:   String,
}

impl PasswordRule {
    pub fn new(unparsed_rule: &String) -> Self {
        let parser = RuleParser::new(unparsed_rule);
        Self{
            min:    parser.min.parse::<usize>().unwrap(),
            max:    parser.max.parse::<usize>().unwrap(),
            letter: parser.letter,
            pass:   parser.pass,
        }
    }

    pub fn is_valid(&self) -> bool {
        let matches = self.pass.chars().fold(0, |count, ch| if ch == self.letter { count + 1 } else { count } );
        return self.min <= matches && matches <= self.max;
    }

    pub fn is_valid_v2(&self) -> bool {
        let ch1 = self.pass.chars().nth(self.min - 1);
        let ch2 = self.pass.chars().nth(self.max - 1);
        let match1 = match ch1 {
            Some(ch) => { ch == self.letter },
            None     => { false }
        };
        let match2 = match ch2 {
            Some(ch) => { ch == self.letter },
            None     => { false }
        };

        return match1 != match2 && (match1 || match2);
    }
}

struct RuleParser {
    min:        String,
    max:        String,
    letter:     char,
    pass:       String,
    mode:       ParseMode,
}

enum ParseMode {
    Min { skip: u8 },
    Max { skip: u8 },
    Letter { skip: u8 },
    Password { skip: u8},
}

// 2-9 c: ccccccccc
impl RuleParser {
    pub fn new(line: &String) -> Self {
        let mut parser = Self{
            min:        String::new(),
            max:        String::new(),
            letter:     'x',
            pass:       String::new(),
            mode:       ParseMode::Min{ skip: 0 },
        };
        parser.parse_line(line);
        return parser;
    }

    fn parse_line(&mut self, line: &String) -> () {
        for ch in line.chars() {
            self.consume_char(ch);
        }
    }

    fn consume_char(&mut self, ch: char) -> () {
        match &self.mode {
            ParseMode::Min { skip }      => { self.append_min(ch, *skip) },
            ParseMode::Max { skip }      => { self.append_max(ch, *skip) },
            ParseMode::Letter { skip }   => { self.set_letter(ch, *skip) },
            ParseMode::Password { skip } => { self.append_pass(ch, *skip) },
        }
    }

    fn append_min(&mut self, ch: char, skip: u8) -> () {
        if skip > 0 {
            self.mode = ParseMode::Min{ skip: skip - 1 };
        } else if ch.is_digit(10) {
            self.min.push(ch);
        } else {
            self.mode = ParseMode::Max{ skip: 0 };
        }
    }

    fn append_max(&mut self, ch: char, skip: u8) -> () {
        if skip > 0 {
            self.mode = ParseMode::Max{ skip: skip - 1 };
        } else if ch.is_digit(10) {
            self.max.push(ch);
        } else {
            self.mode = ParseMode::Letter{ skip: 0 };
        }
    }

    fn set_letter(&mut self, ch: char, skip: u8) -> () {
        if skip > 0 {
            self.mode = ParseMode::Letter{ skip: skip - 1 };
        } else {
            self.letter = ch;
            self.mode = ParseMode::Password{ skip: 2 };
        }
    }

    fn append_pass(&mut self, ch: char, skip: u8) -> () {
        if skip > 0 {
            self.mode = ParseMode::Password{ skip: skip - 1 };
        } else {
            self.pass.push(ch);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ex02::*;

    #[test]
    fn test_is_valid() {
        let rule1 = PasswordRule::new(&String::from("1-3 a: abcde"));
        assert_eq!(rule1.is_valid(), true);

        let rule2 = PasswordRule::new(&String::from("1-3 b: cdefg"));
        assert_eq!(rule2.is_valid(), false);

        let rule3 = PasswordRule::new(&String::from("2-9 c: ccccccccc"));
        assert_eq!(rule3.is_valid(), true);
    }

    #[test]
    fn test_is_valid_v2() {
        let rule1 = PasswordRule::new(&String::from("1-3 a: abcde"));
        assert_eq!(rule1.is_valid_v2(), true);

        let rule2 = PasswordRule::new(&String::from("1-3 b: cdefg"));
        assert_eq!(rule2.is_valid_v2(), false);

        let rule3 = PasswordRule::new(&String::from("2-9 c: ccccccccc"));
        assert_eq!(rule3.is_valid_v2(), false);
    }

    #[test]
    fn test_part_a() {
        if let Some(result) = crate::ex02::part_a(Some(&"./src/ex02/sample.txt".to_string())) {
            assert_eq!(result, 2);
        } else {
            panic!("Could not find a match");
        }
    }
}
