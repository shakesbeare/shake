use shake::*;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Init { cargo, lfs } => init(cargo, lfs),
        SubCommand::New { name, cargo, lfs } => new(name, cargo, lfs),
        SubCommand::Clone { uri, branch } => clone(uri, branch),
    }
}
