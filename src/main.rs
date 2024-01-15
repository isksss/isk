use clap::{Parser, Subcommand};
use isk::subcmd::dot;
use isk::subcmd::paper;
use isk::subcmd::rand;

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

    #[command(about = "dotfiles management")]
    Dot {
        #[command(subcommand)]
        cmd: Option<DotCommands>,
    },

    #[command(about = "random")]
    Rand {
        #[arg(short, long, default_value = "3")]
        num: u32,
    },
}

#[derive(Subcommand)]
enum PaperCommands {
    #[command(about = "download paper")]
    Download,
    #[command(about = "create paper.toml")]
    Init,
}

#[derive(Subcommand)]
enum DotCommands {
    #[command(about = "create dotfiles")]
    Clone {
        #[arg(short, long, default_value = "isksss/dotfiles")]
        repo: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Paper { cmd }) => match cmd {
            Some(PaperCommands::Download) => paper::download(),
            Some(PaperCommands::Init) => paper::create_config(),
            None => {}
        },
        Some(Commands::Dot { cmd }) => match cmd {
            Some(DotCommands::Clone { repo }) => dot::clone_dotfiles(repo),
            None => {}
        },
        Some(Commands::Rand{num}) => {
            rand::rand(*num);
        }

        None => {
            println!("コマンドがありません");
        }
    }
}
