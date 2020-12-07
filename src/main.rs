use std::env;

mod utils;
mod ex01;
mod ex02;
mod ex03;
mod ex04;
mod ex05;
mod ex06;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let prog_num = args[1].as_str();
        let result = match prog_num {
            "1a" => ex01::part_a(args.get(2)),
            "1b" => ex01::part_b(args.get(2)),
            "2a" => ex02::part_a(args.get(2)),
            "2b" => ex02::part_b(args.get(2)),
            "3a" => ex03::part_a(args.get(2)),
            "3b" => ex03::part_b(args.get(2)),
            "4a" => ex04::part_a(args.get(2)),
            "4b" => ex04::part_b(args.get(2)),
            "5a" => ex05::part_a(args.get(2)),
            "5b" => ex05::part_b(args.get(2)),
            "6a" => ex06::part_a(args.get(2)),
            _ => {
                println!("Could not recognize program {}", prog_num);
                None
            }
        };
        if let Some(answer) = result {
            println!("The result is {}", answer);
        } else {
            println!("No solution found for program {}", prog_num);
        }
    } else {
        println!("No program specified");
    }
}
