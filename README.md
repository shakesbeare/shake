# Shake

A project manager for working with git worktrees

# Usage

Shake is a project manager designed to ease the use of git worktrees in your normal git workflow.
The most basic way to use Shake is to create a new project.

```sh
shake new my-fantastic-project
```

Shake enforces a worktree-based project structure which looks like this:

```
./
.git/
main/ 
  README.md
branch/
  README.md
```

Shake also provides a wrapper for cloning repositories to quickly get into action. You can optionally
provide a different branch name to check out than `main`.

```sh
shake clone -b master git@github.com:username/repo.git
```
```
./
.git/
master/
  README.md
```

# Installation

## Build from source 

To install `shake`, clone the repo and run the following command:

```sh
sudo cargo xtask install
```

# Requirements

- MacOS
- Rustup Toolchain

# Planned Features

- Improved help command

