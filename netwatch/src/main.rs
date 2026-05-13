// netwatch — Ch21 Macros | Ch22 Unsafe | Ch23 FFI
//
// USAGE:
//   cargo run -- macros  → Ch21: macro_rules!, token patterns, code generation
//   cargo run -- unsafe  → Ch22: raw pointers, unsafe blocks, safe wrappers
//   cargo run -- ffi     → Ch23: calling C from Rust, CString, extern "C"

#[macro_use]
mod macros;
mod unsafe_ops;
mod ffi;

use std::env;

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- macros");
    println!("  cargo run -- unsafe");
    println!("  cargo run -- ffi");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "macros" => {
            // TODO: call your macro demos here
            // Available macros:
            //   collect_vec![1, 2, 3]
            //   assert_eq_verbose!(left, right)
            //   newtype!(TypeName, inner_type)  — emits a struct definition
        }
        "unsafe" => {
            // TODO: call your unsafe function demos here
            // Available functions:
            //   unsafe_ops::swap(&mut a, &mut b)
            //   unsafe_ops::read_le_u32(bytes)
            //   unsafe_ops::split_at(slice, mid)
        }
        "ffi" => {
            // TODO: call your ffi demos here
            // Available functions:
            //   ffi::c_strlen(s)
            //   ffi::c_abs(n)
            //   ffi::cstring_round_trip(s)
        }
        _ => {
            eprintln!("Unknown: {}", args[1]);
            print_usage();
        }
    }
}
