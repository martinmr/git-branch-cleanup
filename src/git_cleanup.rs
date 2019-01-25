use git2;
use std::error::Error;
use std::io::{self, Write};

use crate::flags::CleanupOptions;

pub fn run_cleanup(options: &CleanupOptions) -> Result<(), Box<Error>> {
    let repo = git2::Repository::open_from_env()?;

    drop_local_only_branches(&repo, &options)?;

    Ok(())
}

fn drop_local_only_branches(
    repo: &git2::Repository,
    options: &CleanupOptions,
) -> Result<(), Box<Error>> {
    let branches = repo.branches(Some(git2::BranchType::Local))?;
    for item in branches {
        let (mut branch, branch_type) = item?;
        if branch_type != git2::BranchType::Local {
            continue;
        }

        let delete_branch = match branch.upstream() {
            Ok(_) => false,
            Err(_) => true,
        };

        if delete_branch {
            drop_branch(&mut branch, options)?;
        }
    }

    Ok(())
}

fn drop_branch(branch: &mut git2::Branch, options: &CleanupOptions) -> Result<(), Box<Error>> {
    let branch_name = branch.name()?.unwrap_or("");

    if options.dry_run {
        println!(
            "Branch {} would be deleted if dry run mode were off.",
            branch_name
        );
        return Ok(());
    }

    let delete = should_drop_branch(&branch_name)?;
    if delete {
        branch.delete()?;
    }

    Ok(())
}

fn should_drop_branch(branch_name: &str) -> Result<bool, Box<Error>> {
    loop {
        let mut input = String::new();
        print!("Delete branch {}? [y/n]: ", branch_name);
        io::stdout().flush().unwrap();
        let _read_bytes = io::stdin().read_line(&mut input)?;
        match input.trim_end_matches('\n') {
            "n" => return Ok(false),
            "y" => return Ok(true),
            _ => continue,
        }
    }
}
