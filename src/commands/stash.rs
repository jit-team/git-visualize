use std::path::PathBuf;

use clap::{
    arg,
    Args,
    Subcommand
};

#[derive(Debug, Clone, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct StashArgs {
    #[command(subcommand)]
    command: Option<StashCommand>,

    #[command(flatten)]
    push: StashPushArgs
}

#[derive(Debug, Clone, Subcommand)]
pub enum StashCommand {
    List,
    Show,
    Drop(StashDropArgs),
    Pop(StashApplyArgs),
    Apply(StashApplyArgs),
    Branch(StashBranchArgs),
    Clear,
    // Create,
    // Store,
    Push(StashPushArgs),
    // Save
}

#[derive(Debug, Clone, Args)]
pub struct StashPushArgs {
    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(short, long)]
    pub patch: bool,

    #[arg(short('S'), long)]
    pub staged: bool,
    
    #[arg(short('k'), long("keep-index"), long("no-keep-index"))]
    pub keep_index: bool,

    #[arg(short, long)]
    pub quiet: bool,
    
    #[arg(short('u'), long)]
    pub include_untracked: bool,
    
    #[arg(short, long)]
    pub all: bool,

    #[arg(long)]
    pub pathspec_from_file: Option<PathBuf>,

    #[arg(long)]
    pub pathspec_file_null: bool,

    pub pathspec: Vec<PathBuf>
}

#[derive(Debug, Clone, Args)]
pub struct StashDropArgs {
    #[arg(short, long)]
    pub quiet: bool,

    pub stash: Option<String>
}

#[derive(Debug, Clone, Args)]
pub struct StashApplyArgs {
    #[arg(long)]
    pub index: u32,

    #[arg(short, long)]
    pub quiet: bool,

    pub stash: Option<String>
}

#[derive(Debug, Clone, Args)]
pub struct StashBranchArgs {
    pub branch_name: String,

    pub stash: Option<String>
}