mod cli;
use clap::{Parser};
fn main() {
    let args = cli::Parameters::parse();
    println!("Show you the Args!　{:?}",args);
}
