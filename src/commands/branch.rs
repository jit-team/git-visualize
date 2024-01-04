use clap::{
    arg,
    Args
};

#[derive(Debug, Clone, Args)]
pub struct BranchArgs {
    // TODO: Need to find a way to distiguish command based on flags

    #[command(flatten)]
    pub delete: BranchDeleteArgs,

    #[command(flatten)]
    pub move_rename: BranchMoveArgs
}

#[derive(Debug, Clone, Args)]
pub struct BranchDeleteArgs {
    #[arg(short, long, conflicts_with("delete_force"))]
    pub delete: bool,

    #[arg(short('D'))]
    pub delete_force: bool,

    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long)]
    pub remotes: bool,

    pub branch_name: Vec<String>
}

#[derive(Debug, Clone, Args)]
pub struct BranchMoveArgs {
    #[arg(short, long("move"))]
    pub move_rename: bool
}