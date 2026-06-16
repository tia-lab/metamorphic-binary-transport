use mbt_codegen::config::{Action, parse_args};
use mbt_codegen::emit;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> mbt_codegen::error::Result<()> {
    // Keep CLI failures typed until the process boundary in main.
    let config = parse_args(std::env::args().skip(1))?;
    match config.action {
        Action::Inspect => {
            let output = emit::inspect(&config)?;
            print!("{output}");
            Ok(())
        }
        Action::Write => emit::write(&config),
        Action::Check => emit::check(&config),
    }
}
