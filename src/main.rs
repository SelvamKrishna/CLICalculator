use std::{ env, process };

use eval::calculator::Calculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("\nUsage: cargo run \"expression\"\n");
        process::exit(1);
    }

    match Calculator::new(args[1].to_string()).eval() {
        Ok(n) => println!("Result: {}", n),
        Err(e) => eprintln!("\n{}\n", e),
    }
}
