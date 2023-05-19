mod repl;
mod tokenizer;

use clap::Parser;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Args {
    filename: Option<std::path::PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    if let Some(_filename) = args.filename {
        Ok(())
    } else {
        repl::run();
        Ok(())
    }
}
