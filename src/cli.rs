use crate::video::Video;

use anyhow::Result;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::debug;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, subcommand_negates_reqs = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
    /// File(s) to get the codec of
    #[arg(required = true)]
    file: Option<Vec<String>>,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completions. Default to current shell
    Completions {
        /// Infer current shell when missing, fallback to bash
        #[clap(value_enum)]
        shell: Option<Shell>,
    },
    /// Walk over all files in a directory
    List {
        /// Root directory to walk
        #[arg(default_value=".")]
        path: String
    },
}

fn generate_completions(shell: Shell, cmd: &mut Command) -> Result<()> {
    generate(
        shell,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    );

    Ok(())
}

fn list_cmd(root: String) -> Result<()> {
    for entry in WalkDir::new(root).into_iter().flatten() {
        if entry.metadata()?.is_file() {
            let path = entry.path().to_str().unwrap();
            let video = Video::new(path);

            match video {
                Ok(video) => println!("{}", video),
                Err(error) => debug!("{}: {}", path, error),
            };
        }
    }

    Ok(())
}

fn default_cmd(paths: Vec<String>) -> Result<()> {
    for path in paths {
        let video = Video::new(&path);

        match video {
            Ok(video) => println!("{}", video),
            Err(error) => debug!("{}: {}", path, error),
        };
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    if let Some(cmd) = args.cmd {
        match cmd {
            Commands::Completions { shell } => {
                let gen = match shell {
                    Some(s) => s,
                    None => Shell::from_env().unwrap_or(Shell::Bash),
                };

                generate_completions(gen, &mut Cli::command())?
            }
            Commands::List { path } => list_cmd(path)?,
        }
    } else {
        default_cmd(args.file.unwrap())?
    }

    Ok(())
}
