# git-global

[![Crates.io](https://img.shields.io/crates/v/git-global.svg)](https://crates.io/crates/git-global)
[![Crates.io](https://img.shields.io/crates/d/git-global.svg)](https://crates.io/crates/git-global)

Use `git-global` to keep track of all your git repositories.

This is a Rust program that you can put on your `PATH` with `cargo install git-global`, gaining an extra git subcommand that you can run from anywhere. To
obtain cargo and Rust, see https://rustup.rs.

Use `git global <subcommand>` to:

- `git global [status]`: show `git status -s` for all your git repos (the
  default subcommand)
- `git global info`: show information about git-global itself (configuration,
  number of known repos, etc.)
- `git global list`: show all git repos git-global knows about
- `git global scan`: search your filesystem for git repos and update cache

## Configuration

To change the behavior of `git-global`, you can do so with --- wait for it
--- git global configuration!

To set the base path for search to something other than your home directory:

```
git config --global global.basedir /some/path
```

To add patterns to exclude while walking directories:

```
git config --global global.ignore .cargo,.vim,Library
```

## TODO

- [x] filter `tag-projects` by path, status, prepopulate a list of potential new tags
- [x] Option to strip all tags
- [ ] Better message suggesting "use list or status"
- [ ] Filter list
  - [x] by regex
  - [ ] by tag
  - [x] showing coloured text output
- [ ] Some tests?
- [ ] Some assertions
- [ ] reconcile `add_tags` and `tag_projects_redo`
- [ ] have a negative filter option for status also (things to exclude)
- [x] Replace mpsc with crossbeam in `status` command

### Rust 2018

- [ ] extern crates
- [ ] try to get rid of some unwraps for `?`s

### Bugs

- [ ] What if we scan for repos again - do we lose all our existing tag info?
  - [ ] Should have an "update repos" scan by default that will keep existing tags info...
  - [ ] We need to store tags separately maybe?
- [x] When we tag-projects after filtering by path/tag and save them we are overwriting our repos with a subset of all our repos

### Bigger Changes

- [ ] Maybe i need to refactor the basic data types - they probably dont make sense for what i want this project to be
  - particularly `GitGlobalConfig` and `GitGlobalResult`
    - `GitGlobalConfig` reads .gitglobalconfig and stores option
      - it is used to read/save repos to the cache
    - `GitGlobalResult` stores a bunch of "messages" for each `Repo` and in general for the result of an operation
  - What would fit instead?
    - Need to persist more data
    - Initial resource should fetch and store our global repo/tag list
      - When we try to save a subset of repos we should update this list/merge the two and then save
    - So we should perhaps store
      - default tags
      - current tags
      - baseline/cached repos
      - subset of total repos
      - baseline/cached/default actions
      - newer actions
- [ ] Get rid of duplicated core functionality

### Actions

- [ ] An option to run an action on groups of repos/directories
- [ ] A whitelist or blacklist of commands
  - i.e. dont run `rm` or `mv` without a prompt
- [ ] A dangerous flag?
- [ ] Perhaps a more complex input format/source than a comma separated list of strings
- [ ] A GUI composer - let you see what Repos you would be about to effect with an action based on path/tag filtering.
- [ ] A way to sequence actions run one then another
- [ ]

## Ideas

- [ ] `git global unstaged`: show all repos that have unstaged changes
- [ ] `git global staged`: show all repos that have staged changes
- [ ] `git global stashed`: show all repos that have stashed changes
- [ ] `git global dirty`: show all repos that have changes of any kind
- [ ] `git global branched`: show all repos not on `master` (TODO: or a different
      default branch in .gitconfig)
- [ ] `git global duplicates`: show repos that are checked out to multiple places
- [ ] `git global remotes`: show all remotes (TODO: why? maybe filter by hostname?)

- [ ] `git global add <path>`: add a git repo to the cache that would not be found in a scan
- [ ] `git global ignore <path>`: ignore a git repo and remove it from the cache
- [ ] `git global ignored`: show which git repos are currently being ignored
- [ ] `git global monitor`: launch a daemon to watch git dirs with inotify
- [ ] `git global pull`: pull down changes from default tracking branch for clean repos

- [ ] stream results as the come in (from `git global status`, for example, so we don't
      have to wait until they're all collected)
- [ ] use `locate .git` if the DB is populated, instead of walking everything
- [ ] make a `Subcommand` trait
- [ ] do concurrency generically, not just for status subcommand
- [ ] rename `GitGlobalResult` so it's not confused with a normal `Result`

## Release Notes

- 0.1.1 (work-in-progress)
  - add tests
  - expand documentation and package metadata
  - update dependency versions
- 0.1.0 (1/31/17)
  - initial release with the following subcommands: help, info, list, scan, status
