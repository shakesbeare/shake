use crate::*;
use std::process::Command;

pub fn init(cargo: bool, go: bool, lfs: bool) -> Result<()> {
    // project file structure
    // project-name/
    //     .git # the bare repository
    //     main/ # worktrees!

    // initialize git
    Command::new("git")
        .args(["init", "--bare", ".git"])
        .output()
        .expect("failed to initialize git");

    let name = std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

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

    if go {
        let package_name = format!("changeme/{}", name);
        Command::new("go")
            .args(["mod", "init", &package_name])
            .spawn()
            .expect("failed to spawn `go mod init`")
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

pub fn new(name: String, cargo: bool, go: bool, lfs: bool) -> Result<()> {
    // create new dir with the project name
    std::fs::create_dir(&name).expect("failed to create new project directory");

    // change to the new directory
    std::env::set_current_dir(&name)
        .expect("failed to change to new project directory");

    init(cargo, go, lfs)?;

    Ok(())
}
