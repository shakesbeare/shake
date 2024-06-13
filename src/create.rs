use crate::*;
use std::process::Command;

pub fn init(cargo: bool, go: bool, lfs: bool, rye: bool) -> Result<()> {
    // project file structure
    // project-name/
    //     .git # the bare repository
    //     main/ # worktrees!

    // initialize git
    Command::new("git")
        .args(["init", "--bare", ".git"])
        .output()?;

    let name = std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // initialize the project folders
    std::fs::create_dir("temp")?;
    std::env::set_current_dir("temp")?;

    Command::new("git").arg("init").output()?;

    std::fs::File::create("README.md")?;

    // handle optional project details
    if cargo {
        Command::new("cargo").arg("init").spawn()?.wait()?;
    }

    if go {
        let package_name = format!("changeme/{}", name);
        Command::new("go")
            .args(["mod", "init", &package_name])
            .spawn()?
            .wait()?;
    }

    if rye {
        Command::new("rye")
            .args(["init", "--script"])
            .spawn()?
            .wait()?;
    }

    if lfs {
        Command::new("git")
            .args(["lfs", "install"])
            .spawn()?
            .wait()?;
    }

    Command::new("git").args(["add", "."]).output()?;
    Command::new("git")
        .args(["commit", "-m", "\"initial commit\""])
        .output()?;
    Command::new("git")
        .args(["remote", "add", "origin", "../.git"])
        .output()?;
    Command::new("git")
        .args(["push", "-u", "origin", "main"])
        .output()?;

    // finish setting up project
    std::env::set_current_dir("..")?;
    std::fs::remove_dir_all("temp")?;
    Command::new("git")
        .args(["worktree", "add", "main"])
        .output()?;

    // remove the temp origin
    std::env::set_current_dir("main")?;
    Command::new("git")
        .args(["remote", "remove", "origin"])
        .output()?;

    Ok(())
}

pub fn new(name: String, cargo: bool, go: bool, lfs: bool, rye: bool) -> Result<()> {
    // create new dir with the project name
    std::fs::create_dir(&name)?;

    // change to the new directory
    std::env::set_current_dir(&name)?;

    init(cargo, go, lfs, rye)?;

    Ok(())
}
