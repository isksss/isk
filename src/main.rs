use clap::{Parser, Subcommand};
use isk::subcmd::paper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "paper management")]
    Paper {
        #[command(subcommand)]
        cmd: Option<PaperCommands>,
    },
}

#[derive(Subcommand)]
enum PaperCommands {
    #[command(about = "download paper")]
    Download,
    #[command(about = "create paper.toml")]
    Init,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Paper { cmd }) => match cmd{
            Some(PaperCommands::Download) => paper::download(),
            Some(PaperCommands::Init) => paper::create_config(),
            None => {}
        },
        None => {}
    }
}
