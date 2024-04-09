use clap::CommandFactory;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    /// Generate the manpage
    #[clap(name = "manpage")]
    Manpage,
}

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?,
    );
    let man = clap_mangen::Man::new(shake::Cli::command());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(out_dir.join("shake.1"), buffer)?;
    Ok(())
}
