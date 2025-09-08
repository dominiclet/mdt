use std::fs::{OpenOptions};

use clap::Parser as ClapParser;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

mod commands;
mod config;
mod parser;
mod printer;

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Mdt {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();
    info!("New invocation");
    let _ = Mdt::parse();
    let ctx = commands::get_context()?;
    commands::show_status(&ctx)?;
    Ok(())
}

fn setup_logger() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./mdt.log")
        .unwrap();

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file)
        .finish();
    subscriber.init();
}
