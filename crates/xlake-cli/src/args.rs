use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Args {
    pub command: Vec<String>,

    #[arg(global = true, long)]
    pub debug: bool,
}
