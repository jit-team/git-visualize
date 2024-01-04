use clap::Parser;

use self::{
    add::AddArgs,
    commit::CommitArgs,
    stash::StashArgs,
    branch::BranchArgs
};

mod add;
mod commit;
mod stash;
mod branch;


#[derive(Parser, Debug)]
#[command(name = "git")]
pub enum  Git {
    Add(AddArgs),
    Commit(CommitArgs),
    Stash(StashArgs),
    Branch(BranchArgs),
    // Merge(),
    // Tag(),
    // Reset(),
    // Revert(),
    // Restore(),
    // Rm(),
    // Mv(),
    // Switch(),
    // Checkout(),
    // Pull(),
    // Fetch(),
    // Push()
}