use shake::create::*;
use shake::*;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let app = Cli::parse();

    let app_result = match app.subcmd {
        SubCommand::Init {
            cargo,
            go,
            lfs,
            rye,
            npm,
        } => init(cargo, go, lfs, rye, npm),
        SubCommand::New {
            name,
            cargo,
            go,
            lfs,
            rye,
            npm,
        } => new(name, cargo, go, lfs, rye, npm),
        SubCommand::Clone { uri, branch } => clone(uri, branch),
        SubCommand::Checkout { branch, b, force } => checkout(branch, b, force),
    };

    if let Err(e) = app_result {
        eprintln!("{}", e);
    }

    Ok(())
}
