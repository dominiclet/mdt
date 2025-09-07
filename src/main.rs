use clap::Parser as ClapParser;

mod parser;
mod printer;
mod config;
mod commands;

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Mdt {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configs: config::Config = config::read_config()?;
    commands::show_status(configs)?;
    Ok(())
}
