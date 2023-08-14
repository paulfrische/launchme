use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short, long, help = "Loads config from file. If the file doesn't exist it gets created.")]
    pub config: Option<String>,
}
