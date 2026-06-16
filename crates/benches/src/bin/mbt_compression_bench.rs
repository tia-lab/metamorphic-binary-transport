use std::io;
use std::path::PathBuf;

use mbt_benches::compression::{BenchResult, run_benchmark};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> BenchResult<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let parsed = parse_args(&args[1..])?;
    let path = run_benchmark(&parsed.report_dir, parsed.smoke, args.join(" "))?;
    println!("{}", path.display());
    Ok(())
}

struct ParsedArgs {
    smoke: bool,
    report_dir: PathBuf,
}

fn parse_args(args: &[String]) -> BenchResult<ParsedArgs> {
    let mut smoke = false;
    let mut report_dir = None;
    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--smoke" => {
                smoke = true;
                idx += 1;
            }
            "--report-dir" => {
                let Some(value) = args.get(idx + 1) else {
                    return Err(io::Error::other(usage()).into());
                };
                report_dir = Some(PathBuf::from(value));
                idx += 2;
            }
            _ => return Err(io::Error::other(usage()).into()),
        }
    }
    match report_dir {
        Some(path) => Ok(ParsedArgs {
            smoke,
            report_dir: path,
        }),
        None => Err(io::Error::other(usage()).into()),
    }
}

fn usage() -> &'static str {
    "usage: mbt_compression_bench [--smoke] --report-dir <path>"
}
