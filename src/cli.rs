use crate::video::Video;

use anyhow::Result;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, subcommand_negates_reqs = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
    #[arg(required = true)]
    path: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completions. Default to current shell
    Completions {
        /// Infer current shell when missing, fallback to bash
        #[clap(value_enum)]
        shell: Option<Shell>,
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

fn default_cmd(paths: Vec<String>) -> Result<()> {
    for path in paths {
        let video = Video::new(&path);

        match video {
            Ok(video) => println!(
                "{}\t{}",
                video.codec.green().bold(),
                video.filename.purple().bold()
            ),
            Err(error) => eprintln!("{}: {}: {}", "Error".red(), path, error),
        };
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Cli::parse();

    if let Some(cmd) = args.cmd {
        match cmd {
            Commands::Completions { shell } => {
                let gen = match shell {
                    Some(s) => s,
                    None => Shell::from_env().unwrap_or(Shell::Bash),
                };

                generate_completions(gen, &mut Cli::command())?
            }
        }
    } else {
        default_cmd(args.path.unwrap())?
    }

    Ok(())
}
