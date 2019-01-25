# git-branch-cleanup

Utility to delete branches that are no longer in remote.

## Installation

Just make sure the binary is in your execution path. The repo is automatically detected based on the current environment (i.e your current working directory).

## Usage

```
cd <path_to_repo>
git remote update --prune <remote>
git-branch-cleanup
```

## TODO

* Add support for pruning so that running ```git remote``` beforehand is not necessary.
