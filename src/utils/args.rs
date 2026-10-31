use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short='c', long="config")]
    pub config: String
}