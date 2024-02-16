mod cli;
mod video;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
