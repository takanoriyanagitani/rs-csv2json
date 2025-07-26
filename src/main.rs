use std::env;
use std::io;
use std::path::Path;

use rs_csv2json::csv2json2stdout;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let max_lines: usize = env::var("MAX_LINES")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .expect("Invalid value for MAX_LINES");

    csv2json2stdout(Path::new(filename), max_lines)
}
