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
my-fantastic-project/
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
repo/
  .git/
  master/
    README.md
```

You can also use shake to ease the creation of new worktrees with an interface that is similar
to `git checkout`.

```sh
shake checkout my-cool-branch
shake checkout brand-new-branch # doesn't work!
shake checkout -b brand-new-branch
shake checkout -b existing-branch # doesn't work!
shake checkout -bf existing-branch # new branch shadows existing branch
```
```
./
.git/
main/
  README.md
my-cool-branch/
    README.md
brand-new-branch/
    README.md
existing-branch/
    README.md
```

# Installation

## Build from source 

To install `shake`, clone the repo and run the following command:

```sh
sudo cargo xtask install
```

# Requirements

- MacOS (for manpage installation) # support for other operating systems should be trivial to implement
- Cargo
