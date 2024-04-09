use std::path::PathBuf;

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
    /// Install shake to the system
    #[clap(name = "install")]
    Install,
}

fn main() -> std::io::Result<()> {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Manpage => build_manpage()?,
        SubCommand::Install => install()?,
    }

    Ok(())
}

fn build_manpage() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?,
    );
    let man = clap_mangen::Man::new(shake::Cli::command());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(out_dir.join("shake.1"), buffer)?;

    let _ = std::process::Command::new("gzip")
        .arg(out_dir.join("shake.1"))
        .spawn()
        .expect("failed to spawn gzip")
        .wait();

    Ok(())
}

fn install() -> std::io::Result<()> {
    use std::process::Command;
    let built_man_dir = std::env::current_dir()?;
    std::env::set_var("OUT_DIR", &built_man_dir);

    let manpath = {
        let os = std::env::consts::OS;
        match os {
            "macos" => PathBuf::from("/usr/local/share/man/man1"),
            os => panic!("Unsupported OS, {}", os),
        }
    };

    // 1) cargo install shake
    // 2) build manpage
    // 3) copy manpage to /usr/local/man/man1/
    Command::new("cargo")
        .args(vec!["install", "--path", "."])
        .spawn()
        .expect("failed to spawn `cargo install`")
        .wait()?;
    Command::new("cargo")
        .args(vec!["xtask", "manpage"])
        .spawn()
        .expect("failed to spawn `cargo xtask manpage`")
        .wait()?;
    std::fs::copy(
        built_man_dir.join("shake.1.gz"),
        manpath.join("shake.1.gz"),
    )?;

    std::fs::remove_file(built_man_dir.join("shake.1.gz"))?;

    Ok(())
}
