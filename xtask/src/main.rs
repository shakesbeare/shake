use std::path::PathBuf;
use std::process::Stdio;

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
    /// Run tests
    #[clap(name = "test")]
    Test,
}

fn main() -> std::io::Result<()> {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Manpage => build_manpage()?,
        SubCommand::Install => install()?,
        SubCommand::Test => test(),
    }

    Ok(())
}

fn build_manpage() -> std::io::Result<()> {
    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
    let man = clap_mangen::Man::new(shake::Cli::command());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(out_dir.join("shake.1"), buffer)?;

    std::process::Command::new("gzip")
        .arg(out_dir.join("shake.1"))
        .spawn()
        .expect("failed to spawn gzip")
        .wait()?;

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
        .args(["install", "--path", "."])
        .spawn()
        .expect("failed to spawn `cargo install`")
        .wait()?;
    Command::new("cargo")
        .args(["xtask", "manpage"])
        .spawn()
        .expect("failed to spawn `cargo xtask manpage`")
        .wait()?;
    std::fs::copy(built_man_dir.join("shake.1.gz"), manpath.join("shake.1.gz"))?;

    std::fs::remove_file(built_man_dir.join("shake.1.gz"))?;

    Ok(())
}

fn test() {
    use std::process::Command;
    let manifest = std::fs::read_to_string("./Cargo.toml").unwrap();
    shake::setup_test_dir().unwrap();
    assert!(std::fs::exists("/tmp/shake").unwrap());
    let current_dir = std::env::current_dir().unwrap();

    if Command::new("cargo")
        .args(["test", "--manifest-path", current_dir.join("Cargo.toml").to_str().unwrap(), "--", "--test-threads=1"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .is_err()
    {
        println!("An error occurred while attempting to run tests");
    }
    std::env::set_current_dir(current_dir).unwrap();
    std::fs::remove_dir_all("/tmp/shake").unwrap();
    std::fs::write("./Cargo.toml", &manifest).unwrap();
}
