use clap::Parser;
use std::error::Error;

mod commands {
    pub mod gen;
    pub mod get;
    pub mod launch;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub gen: bool,
    
}