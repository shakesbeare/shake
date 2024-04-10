use shake::*;
use shake::create::*;

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Init { cargo, go, lfs } => init(cargo, go, lfs),
        SubCommand::New { name, cargo, go, lfs } => new(name, cargo, go, lfs),
        SubCommand::Clone { uri, branch } => clone(uri, branch),
        SubCommand::Checkout { branch, b, force } => checkout(branch, b, force),
    }
}
