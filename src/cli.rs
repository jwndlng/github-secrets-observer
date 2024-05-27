use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub organization: Option<String>,
    #[arg(short, long)]
    pub disable_secret_logging: bool,
}
