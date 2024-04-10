
use clap::Parser;
use std::process::Command;
use anyhow::Result;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Create a new project in the current directory
    #[clap(name = "init")]
    Init {
        /// Initialize the project with cargo
        #[arg(long = "cargo")]
        cargo: bool,
        /// Install git-lfs into the project
        #[arg(long = "lfs")]
        lfs: bool,
    },
    /// Create a new project in a directory with the given name
    #[clap(name = "new")]
    New {
        name: String,
        /// Initialize the project with cargo
        #[arg(long = "cargo")]
        cargo: bool,
        /// Install git-lfs into the project
        #[arg(long = "lfs")]
        lfs: bool,
    },
    /// Create a project structure with the project at the given uri
    #[clap(name = "clone")]
    Clone { 
        uri: String,
        /// Checkout a specific branch for the first worktree
        #[arg(long = "branch", short, default_value_t = String::from("main"))]
        branch: String,
    },
}

pub fn init(cargo: bool, lfs: bool) -> Result<()> {
    // project file structure
    // project-name/
    //     .git # the bare repository
    //     main/ # worktrees!

    // initialize git
    Command::new("git")
        .args(["init", "--bare", ".git"])
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

    // handle optional project details
    if cargo {
        Command::new("cargo")
            .arg("init")
            .spawn()
            .expect("failed to spawn `cargo init`")
            .wait()?;
    }

    if lfs {
        Command::new("git")
            .args(["lfs", "install"])
            .spawn()
            .expect("failed to install git-lfs")
            .wait()?;
    }

    Command::new("git")
        .args(["add", "."])
        .output()
        .expect("failed to add temp dir to git repository");
    Command::new("git")
        .args(["commit", "-m", "\"initial commit\""])
        .output()
        .expect("failed to commit to git");
    Command::new("git")
        .args(["remote", "add", "origin", "../.git"])
        .output()
        .expect("failed to add git remote");
    Command::new("git")
        .args(["push", "-u", "origin", "main"])
        .output()
        .expect("failed to push to remote");

    // finish setting up project
    let _ = std::env::set_current_dir("..");
    Command::new("rm")
        .args(["-rf", "temp"])
        .output()
        .expect("failed to remove temp dir");
    Command::new("git")
        .args(["worktree", "add", "main"])
        .output()
        .expect("failed to initialize git worktree");

    // remove the temp origin
    let _ = std::env::set_current_dir("main");
    Command::new("git")
        .args(["remote", "remove", "origin"])
        .output()
        .expect("failed to remove temporary origin");

    Ok(())
}

pub fn new(name: String, cargo: bool, lfs: bool) -> Result<()> {
    // create new dir with the project name
    std::fs::create_dir(&name).expect("failed to create new project directory");

    // change to the new directory
    std::env::set_current_dir(&name)
        .expect("failed to change to new project directory");

    init(cargo, lfs)?;

    Ok(())
}

pub fn clone(uri: String, branch: String) -> Result<()> {
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
        .args(["clone", "--bare", &uri, ".git"])
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
        .args(["worktree", "add", &branch])
        .spawn()
        .expect("failed to checkout main worktree")
        .wait();

    match worktree_status {
        Ok(s) => {
            if !s.success() {
                // clean up folder
                let _ = std::env::set_current_dir("..");
                let _ = std::fs::remove_dir_all(format!("./{}", folder_name));
                eprintln!("could not create worktree");
                std::process::exit(1);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "An error occurred while attempting to create the worktree {}",
                e
            );

            Err(e.into())
        }
    }
}
