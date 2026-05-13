// capstone-01 — rgrep: a ripgrep-like search tool
//
// ── HOW IT ALL CONNECTS ───────────────────────────────────────
//
//   Config::parse()            clap reads CLI args into Config
//       ↓
//   search::build_regex        compile Config.pattern → Regex
//       ↓
//   walker::build_walker       walk Config.paths → file paths iterator
//       ↓  (parallel with rayon)
//   search::search_file        each file → Vec<Match>
//       ↓
//   output::print_match        format + colorize + print each Match
//   output::print_count        (if --count-only) print per-file totals
//
//   run() in lib.rs ties steps 2-5 together and returns total match count.
//
// ── IMPLEMENTATION ORDER ──────────────────────────────────────
//
//   1. matcher.rs   — Match::new, has_matches
//                     (pure data; no external deps — start here)
//
//   2. search.rs    — build_regex(pattern, ignore_case) → Result<Regex>
//                     search_file(path, contents, regex) → Vec<Match>
//                     (needs matcher + the `regex` crate)
//
//   3. walker.rs    — build_walker(paths, follow_links) → impl Iterator
//                     collect_paths(paths, follow_links) → Vec<PathBuf>
//                     (needs the `ignore` crate for .gitignore support)
//
//   4. output.rs    — highlight_line(line, ranges, color) → String
//                     print_match(m, show_line_numbers, color)
//                     print_count(path, count, color)
//                     (needs the `colored` crate for ANSI colors)
//
//   5. lib.rs run() — walk all files → read each → search → print
//                     run in parallel with rayon, return total count
//
// ── ONCE COMPLETE: try it ─────────────────────────────────────
//
//   cargo run -- fn src/
//   cargo run -- "pub fn" src/ --line-numbers
//   cargo run -- TODO . --ignore-case --count

use clap::Parser;
use grep_clone::{Config, run};

fn main() {
    let config = Config::parse();
    match run(&config) {
        Ok(count) => {
            if count == 0 {
                std::process::exit(1); // grep convention: exit 1 when no matches
            }
        }
        Err(e) => {
            eprintln!("rgrep: {}", e);
            std::process::exit(2);
        }
    }
}
