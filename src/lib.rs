pub mod create;

use anyhow::{Context, Result};
use clap::Parser;
use std::{path::Path, process::Command};

#[derive(Debug, Parser)]
#[command(name = "Shake")]
#[command(version, about)]
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
        /// Initialize the project with go
        #[arg(long = "go")]
        go: bool,
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
        /// Initialize the project with go
        #[arg(long = "go")]
        go: bool,
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
    /// Checkout a branch using worktrees
    #[clap(name = "checkout")]
    Checkout {
        /// The name of the branch to checkout as a worktree
        branch: String,
        /// Create the branch
        #[arg(short)]
        b: bool,
        /// Only useful with -b. Force branch creation to occur even if the branch already exists
        #[arg(long = "force", short)]
        force: bool,
    },
}

struct CommandBuilder<'a> {
    args: Vec<&'a str>,
}

impl<'a> CommandBuilder<'a> {
    fn new() -> Self {
        CommandBuilder { args: vec![] }
    }
    fn arg(&mut self, arg: &'a str) {
        self.args.push(arg);
    }

    fn args(&mut self, args: &[&'a str]) {
        self.args.extend(args);
    }

    fn into_command(self) -> Command {
        let mut command = Command::new(self.args[0]);
        for arg in self.args.iter().skip(1) {
            command.arg(arg);
        }

        command
    }
}

trait HasGit {
    fn has_git(&self) -> bool;
}

impl HasGit for Path {
    fn has_git(&self) -> bool {
        for file in self.read_dir().unwrap() {
            let Ok(f) = file else {
                return false;
            };
            if f.file_name() == ".git" && f.file_type().unwrap().is_dir() {
                return true;
            }
        }
        return false;
    }
}

pub fn checkout(branch: String, b: bool, force: bool) -> Result<()> {
    // if -b flag is present, simply add the worktree
    let mut builder = CommandBuilder::new();
    let project_path = {
        // search upwards for .git folder
        let cwd = std::env::current_dir()?;
        let mut git_dir = cwd.as_path();
        while !git_dir.has_git() {
            git_dir = git_dir.parent().context("dir should have a parent")?;
        }
        git_dir.to_path_buf()
    };
    let path = project_path.join(&branch);
    let path = path.to_string_lossy();
    if b {
        builder.args(&["git", "worktree", "add"]);
        if force {
            builder.arg("-f");
        }
        builder.arg(&path);
    } else {
        builder.args(&["git", "worktree", "add", "-f", &path, &branch]);
    }

    builder.into_command().spawn()?.wait()?;

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
    std::fs::create_dir(folder_name)?;
    std::env::set_current_dir(folder_name)?;

    let clone_status = Command::new("git")
        .args(["clone", "--bare", &uri, ".git"])
        .spawn()?
        .wait();

    match clone_status {
        Ok(s) => {
            if !s.success() {
                // clean up folder
                std::env::set_current_dir("..")?;
                std::fs::remove_dir_all(format!("./{}", folder_name))?;
            }
        }
        Err(e) => {
            eprintln!("An error occurred while attempting to clone {}", e);
        }
    }

    let worktree_status = Command::new("git")
        .args(["worktree", "add", &branch])
        .spawn()?
        .wait();

    match worktree_status {
        Ok(s) => {
            if !s.success() {
                // clean up folder
                std::env::set_current_dir("..")?;
                std::fs::remove_dir_all(format!("./{}", folder_name))?;
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
