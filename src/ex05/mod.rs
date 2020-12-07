use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> Option<u32> {
    let tickets = match filename {
        Some(path) => parse_tickets(path),
        None => { return None; }
    };
    match tickets.iter().max() {
        Some(max) => Some(*max as u32),
        None => None,
    }
}

pub fn part_b(filename: Option<&String>) -> Option<u32> {
    let tickets = match filename {
        Some(path) => parse_tickets(path),
        None => { return None; }
    };
    match tickets.iter().max() {
        Some(max) => {
            match tickets.iter().min() {
                Some(min) => {
                    find_missing_seat(&tickets, *min, *max)
                },
                None => None
            }
        },
        None => None
    }
}

fn find_missing_seat(tickets: &Vec<u16>, min: u16, max: u16) -> Option<u32> {
    let ticket_sublist_opt = tickets.get(0..(tickets.len() - 1));
    match ticket_sublist_opt {
        Some(ticket_sublist) => {
            for (i, ticket) in ticket_sublist.iter().enumerate() {
                if ticket + (1 as u16) != tickets[i + 1] { return Some(*ticket as u32); }
            }
        },
        None => ()
    }
    None
}

pub fn parse_tickets(path: &String) -> Vec<u16> {
    match read_lines(path) {
        Ok(lines) => {
            let mut tickets = lines.map(|line_r| {
                match line_r {
                    Ok(line) => parse_ticket(line),
                    _ => 0,
                }
            }).collect::<Vec<_>>();
            tickets.sort();
            println!("{}", tickets.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
            tickets
        },
        Err(_e) => { vec![] }
    }
}

pub fn parse_ticket(ticket_str: String) -> u16 {
    ticket_str.chars().fold(0, |sum, ch| {
        match ch {
            'B' => sum * 2 + 1,
            'R' => sum * 2 + 1,
            _   => sum * 2,
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::ex05::*;

    #[test]
    fn test_ticket_parser() {
        assert_eq!(parse_ticket("BFFFBBFRRR".to_string()), 567);
    }

    #[test]
    fn test_tickets_parser() {
        let tickets = parse_tickets(&"./src/ex05/sample.txt".to_string());
        assert_eq!(tickets, vec![567, 119, 820]);
    }

    fn test_part_a() {
    }
}

