use shake::*;
use clap::Parser;

fn main() {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Init { cargo, lfs } => init(cargo, lfs),
        SubCommand::New { name, cargo, lfs } => new(name, cargo, lfs),
        SubCommand::Clone { uri, branch } => clone(uri, branch),
    }
}
