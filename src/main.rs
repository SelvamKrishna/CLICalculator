use std::{ env, process };

use eval::calculator::Calculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("\nUsage: cargo run \"expression\"\n");
    }

    let mut calc = Calculator::new(args[1].to_string());
    let result = calc.eval();

    if let Err(e) = &result {
        eprintln!("\n{}\n", e);
        process::exit(1);
    }

    println!("Result: {}", result.ok().unwrap());
}
