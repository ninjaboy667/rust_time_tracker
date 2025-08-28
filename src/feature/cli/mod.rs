use clap::{Parser, Subcommand};
use error_stack::Result;

#[derive(Debug, thiserror::Error)]
#[error("a CLI error occured")]
pub struct CliError;

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Start tracking time
    Start,
    Stop,
    Delete,
    Report
}
// no command --> error
// command -> no error
#[derive(Debug, Parser)]
#[command(version, about, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}


pub fn run() -> Result<(), CliError> {
    println!("Running CLI... ");
    println!("Available commands: start, stop, report");
    
    let args = Cli::parse();

    match args.command {
        Command::Start => todo!("Start tracking time"),
        Command::Stop => todo!(),
        Command::Delete => todo!(),
        Command::Report => todo!(),
    }

    Ok(())
}