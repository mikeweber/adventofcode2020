use std::env;

mod ex1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let prog_num = args[1].as_str();
        let result = match prog_num {
            "1a" => ex1::part_a(args.get(2)),
            _ => {
                println!("Could not recognize program {}", prog_num);
                0
            }
        };
        println!("The result is {}", result);
    } else {
        println!("No program specified");
    }
}
