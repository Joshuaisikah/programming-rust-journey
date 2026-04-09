// ============================================================
// main.rs — Entry Point (keep this thin)
// ============================================================
//
// Responsibility: parse CLI args and dispatch to the right module.
// All logic lives in lib.rs modules — not here.
//
// 🎓 C# NOTE:
//   This is like Program.cs in a minimal C# app.
//   Keep it thin: parse args, call functions, handle top-level errors.
//
// STRUCTURE:
//   1. Get args: env::args().collect()
//   2. If less than 2 args → call display::print_usage() and return
//   3. Match on args[1]:
//        "convert" → parse_number(args[2]) + convert_base(num, args[3])
//        "temp"    → convert_temperature(args[2], args[3], args[4])
//        _         → parse_number(args[1..3]) + calculate(a, op, b)

use std::env;
use ch03_fundamental_types::{
    calculate, convert_base, convert_temperature,
    display::print_usage, parse_number,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip the program name (args[0])
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "convert" => {
            if args.len() < 4 {
                eprintln!("Error: convert requires 2 arguments: <number> <base>");
                print_usage();
                return;
            }
            let num = match parse_number(&args[2]) {
                num => num,
                // parse_number already panics on invalid input, so this shouldn't happen
            };
            convert_base(num, &args[3]);
        }
        "temp" => {
            if args.len() < 5 {
                eprintln!("Error: temp requires 3 arguments: <value> <from> <to>");
                print_usage();
                return;
            }
            let value = match args[2].parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Error: invalid temperature value '{}'", args[2]);
                    return;
                }
            };
            convert_temperature(value, &args[3], &args[4]);
        }
        _ => {
            // Default: arithmetic calculation
            if args.len() < 4 {
                eprintln!("Error: calculation requires 3 arguments: <num1> <op> <num2>");
                print_usage();
                return;
            }
            let a = match parse_number(&args[1]) {
                a => a,
                // parse_number already panics on invalid input
            };
            let b = match parse_number(&args[3]) {
                b => b,
                // parse_number already panics on invalid input
            };
            calculate(a, &args[2], b);
        }
    }
}
