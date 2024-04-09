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
    #[clap(name = "clone")]
    Clone { uri: String },
}

// project file structure
// project-name/
//     .git # the bare repository
//     main/ # worktrees!

fn init() {
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

    // remove the temp origin
    let _ = std::env::set_current_dir("main");
    Command::new("git")
        .arg("remote")
        .arg("remove")
        .arg("origin")
        .output()
        .expect("failed to remove temporary origin");
}

fn new(name: String) {
    // create new dir with the project name
    std::fs::create_dir(&name).expect("failed to create new project directory");

    // change to the new directory
    std::env::set_current_dir(&name)
        .expect("failed to change to new project directory");

    init();
}

fn clone(uri: String) {
    // get folder name from repo string
    // format: git@github.com:username/repo.git
    //     or: host:username/repo.git

    let Some((_, user_repo)) = uri.split_once(':') else {
        eprintln!("Invalid repo URI\nFormat: git@github.com:username/repo.git");
        std::process::exit(1);
    };
    let Some((_, repo)) = user_repo.split_once('/') else {
        eprintln!("Invalid repo URI\nFormat: git@github.com:username/repo.git");
        std::process::exit(1);
    };
    let folder_name = repo.trim_end_matches(".git");
    let _ = std::fs::create_dir(folder_name);
    let _ = std::env::set_current_dir(folder_name);

    let clone_status = Command::new("git")
        .arg("clone")
        .arg("--bare")
        .arg(&uri)
        .arg(".git")
        .spawn()
        .expect("failed to spawn `git clone` child process")
        .wait();

    match clone_status {
        Ok(s) => {
            if !s.success() {
                // clean up folder
                let _ = std::env::set_current_dir("..");
                let _ = std::fs::remove_dir_all(format!("./{}", folder_name));
            }
        }
        Err(e) => {
            eprintln!("An error occurred while attempting to clone {}", e);
        }
    }

    let worktree_status = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg("main")
        .spawn()
        .expect("failed to checkout main worktree")
        .wait();

    match worktree_status {
        Ok(s) => {
            if !s.success() {
                // clean up folder
                let _ = std::env::set_current_dir("..");
                let _ = std::fs::remove_dir_all(format!("./{}", folder_name));
            }
        }
        Err(e) => {
            eprintln!(
                "An error occurred while attempting to create the worktree {}",
                e
            );
        }
    }
}

fn main() {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Init => init(),
        SubCommand::New { name } => new(name),
        SubCommand::Clone { uri } => clone(uri),
    }
}
