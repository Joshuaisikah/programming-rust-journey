// ============================================================
// converter.rs — Base & Temperature Conversions
// ============================================================
//
// Responsibility: convert numbers between bases, and convert
// temperatures between Celsius, Fahrenheit, and Kelvin.
//
// 🎓 C# NOTE:
//   Use f64 here (not f32) — it's C#'s double.
//   f32 only has ~7 significant digits; f64 has ~15.
//   Float literals must have decimal point: 32.0 not 32.
//
// GOTCHA: floats are never exact!
//   0.1_f64 + 0.2_f64 == 0.3_f64  →  FALSE
//   Use (a - b).abs() < 1e-10 for comparisons in tests.

/// Convert a number to the specified base and print it.
///
/// Accepts full names or abbreviations:
///   "binary" | "bin" | "b"
///   "hex" | "hexadecimal" | "h"
///   "octal" | "oct" | "o"
///   "decimal" | "dec" | "d"
pub fn convert_base(num: i64, to_base: &str) {
    // TODO: implement
    //
    // match to_base.to_lowercase().as_str() {
    //     "binary" | "bin" | "b"         => println!("0b{:b}", num),
    //     "hex" | "hexadecimal" | "h"    => println!("0x{:X}", num),
    //     "octal" | "oct" | "o"          => println!("0o{:o}", num),
    //     "decimal" | "dec" | "d"        => println!("{}", num),
    //     _ => eprintln!("Unknown base: {}. Use binary/hex/octal/decimal", to_base),
    // }
    match to_base.to_lowercase().as_str(){
        "binary" | "bin" | "b" => println!("0b{:b}", num),
        "hex" | "h" | "hexadecimal" => println!("0x{:x}", num),
        "octal" | "o" | "oct" => println!("0o{:o}", num),
        "decimal" | "dec" | "d"        => println!("{}", num),  // ✓ Just print the number
        _ => eprintln!("Unknown base: {}. Use binary/hex/octal/decimal", to_base),
    }
}

/// Convert a temperature value between C, F, and K.
///
/// Strategy: input → Celsius → output (two-step conversion).
/// Prints with 2 decimal places: {:.2}
///
/// # Examples
/// ```
/// use ch03_fundamental_types::converter::convert_temperature;
/// convert_temperature(100.0, "C", "F"); // prints 212.00°C = 212.00°F
/// convert_temperature(0.0, "C", "K");   // prints 273.15 K
/// ```
pub fn convert_temperature(value: f64, from: &str, to: &str) {
    let celcius = to_celsius(value,from);
    let result = celsius_to(celcius, to);
    println!("{:.2}°{} = {:.2}°{}", value, from.to_uppercase(), result, to.to_uppercase());

}
fn celsius_to(celsius: f64, to: &str) -> f64 {
    match to.to_uppercase().as_str() {
        "C" => celsius,
        "F" => (celsius * 9.0 / 5.0) + 32.0,
        "K" => celsius + 273.15,
        _ => panic!("Unknown target unit: {}. Use C/F/K", to),
    }
}

fn to_celsius(value: f64, from: &str) -> f64 {
    match from.to_uppercase().as_str() {
        "C" =>value,
        "F" =>(value -32.0)*5.0/9.0,
        "K" => value-273.15,
        _ => panic!("Unknown target unit: {}. Use C/F/K", from),

    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freezing_point() {
        // 0°C = 32°F = 273.15K
        assert_eq!(celsius_to(0.0, "F"), 32.0);
        assert_eq!(celsius_to(0.0, "K"), 273.15);
        assert!((to_celsius(32.0, "F") - 0.0).abs() < 1e-10);
        assert!((to_celsius(273.15, "K") - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_boiling_point() {
        // 100°C = 212°F = 373.15K
        assert_eq!(celsius_to(100.0, "F"), 212.0);
        assert_eq!(celsius_to(100.0, "K"), 373.15);
        assert!((to_celsius(212.0, "F") - 100.0).abs() < 1e-10);
        assert!((to_celsius(373.15, "K") - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_body_temp() {
        // 37°C ≈ 98.6°F
        assert!((celsius_to(37.0, "F") - 98.6).abs() < 1e-10);
    }
}
