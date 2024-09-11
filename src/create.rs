use crate::*;
use std::process::Command;

pub fn init(cargo: bool, go: bool, lfs: bool, rye: bool, npm: bool, dotnet: bool) -> Result<()> {
    // project file structure
    // project-name/
    //     .git # the bare repository
    //     main/ # worktrees!

    // initialize git
    Command::new("git")
        .args(["init", "--bare", ".git", "--initial-branch", "main"])
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

    Command::new("git").args(["init", "--initial-branch", "main"]).output()?;

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

    if npm {
        Command::new("npm").args(["init", "-y"]).spawn()?.wait()?;
    }

    if dotnet {
        Command::new("dotnet").arg("new").spawn()?.wait()?;
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

    std::env::set_current_dir("..")?;

    Ok(())
}

pub fn new(
    name: String,
    cargo: bool,
    go: bool,
    lfs: bool,
    rye: bool,
    npm: bool,
    dotnet: bool,
) -> Result<()> {
    std::fs::create_dir_all(&name)?;
    std::env::set_current_dir(&name)?;
    init(cargo, go, lfs, rye, npm, dotnet)?;
    std::env::set_current_dir("..")?;

    Ok(())
}

mod test {
    #[allow(unused)]
    #[cfg(debug_assertions)]
    fn print_dir(path: &str) -> anyhow::Result<()> {
        use std::fs::*;

        for entry in read_dir(path)?.flatten() {
            println!("{:?}", entry);
            if entry.metadata()?.is_dir() {
                print_dir(entry.path().to_str().unwrap())?;
            }
        }

        Ok(())
    }

    #[test]
    fn test_init() {
        use std::fs::*;

        std::env::set_current_dir("/tmp/shake").unwrap();
        let cur = std::env::current_dir().unwrap();
        create_dir("test_init").unwrap();
        std::env::set_current_dir("./test_init").unwrap();
        super::init(false, false, false, false, false, false).unwrap();
        print_dir(".").unwrap();

        assert!(exists(".git").unwrap());
        // assert!(exists("main").unwrap());
        assert!(metadata(".git").unwrap().is_dir());
        assert!(metadata("main").unwrap().is_dir());

        assert!(exists("main/README.md").unwrap());
        assert!(metadata("main/README.md").unwrap().is_file());

        std::env::set_current_dir(cur).unwrap();
    }

    #[test]
    fn test_new() {
        use std::fs::*;

        std::env::set_current_dir("/tmp/shake").unwrap();
        let cur = std::env::current_dir().unwrap();

        super::new("test_new".to_string(), false, false, false, false, false, false).unwrap();
        print_dir(".").unwrap();

        assert!(exists("test_new").unwrap());
        assert!(metadata("test_new").unwrap().is_dir());
        std::env::set_current_dir("test_new").unwrap();

        assert!(exists(".git").unwrap());
        // assert!(exists("main").unwrap());
        assert!(metadata(".git").unwrap().is_dir());
        assert!(metadata("main").unwrap().is_dir());

        assert!(exists("main/README.md").unwrap());
        assert!(metadata("main/README.md").unwrap().is_file());

        std::env::set_current_dir(cur).unwrap();
    }

    #[test]
    fn test_cargo() {
        use std::fs::*;

        std::env::set_current_dir("/tmp/shake").unwrap();
        let cur = std::env::current_dir().unwrap();

        super::new("test_cargo".to_string(), true, false, false, false, false, false).unwrap();

        assert!(exists("test_cargo").unwrap());
        assert!(metadata("test_cargo").unwrap().is_dir());
        std::env::set_current_dir("test_cargo/main").unwrap();

        assert!(exists("src").unwrap());
        assert!(exists("Cargo.toml").unwrap());
        assert!(exists(".gitignore").unwrap());

        assert!(metadata("src").unwrap().is_dir());
        assert!(metadata("Cargo.toml").unwrap().is_file());
        assert!(metadata(".gitignore").unwrap().is_file());

        std::env::set_current_dir(cur).unwrap();
    }
}
