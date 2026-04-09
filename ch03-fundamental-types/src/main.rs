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
    // TODO: implement dispatch logic (see structure above)
    todo!("implement main dispatch")
}
