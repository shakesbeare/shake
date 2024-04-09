use clap::Parser;
use std::process::Command;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[clap(name = "init")]
    Init,
    #[clap(name = "new")]
    New { name: String },
}

// project file structure
// project-name/
//     .git # the bare repository
//     main/ # worktrees!

fn init() {
    // get the name of the current directory and use it as the project name
    let _ = std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // initialize git
    Command::new("git")
        .arg("init")
        .arg("--bare")
        .arg(".git")
        .output()
        .expect("failed to initialize git");

    // initialize the project folders
    std::fs::create_dir("temp").expect("failed to create temp dir");
    std::env::set_current_dir("temp").expect("failed to change to a temp dir");

    Command::new("git")
        .arg("init")
        .output()
        .expect("failed to initialize temp inner repository");

    std::fs::File::create("README.md").expect("failed to create README.md");

    Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("failed to add temp dir to git repository");
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("\"initial commit\"")
        .output()
        .expect("failed to commit to git");
    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg("../.git")
        .output()
        .expect("failed to add git remote");
    Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main")
        .output()
        .expect("failed to push to remote");

    // finish setting up project
    let _ = std::env::set_current_dir("..");
    Command::new("rm")
        .arg("-rf")
        .arg("temp")
        .output()
        .expect("failed to remove temp dir");
    Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg("main")
        .output()
        .expect("failed to initialize git worktree");
}

fn new(name: String) {
    // create new dir with the project name
    std::fs::create_dir(&name).expect("failed to create new project directory");

    // change to the new directory
    std::env::set_current_dir(&name)
        .expect("failed to change to new project directory");

    init();
}

fn main() {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Init => init(),
        SubCommand::New { name } => new(name),
    }
}
