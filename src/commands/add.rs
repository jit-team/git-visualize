use std::path::PathBuf;

use clap::{
    arg,
    Args
};

#[derive(Debug, Clone, Args)]
pub struct  AddArgs {
    pub path: Vec<PathBuf>,

    #[arg(short('n'), long)]
    pub dry_run: bool,

    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long)]
    pub interactive: bool,

    #[arg(short, long)]
    pub patch: bool,

    #[arg(short, long)]
    pub edit: bool,
    
    #[arg(short, long)]
    pub update: bool,
    
    #[arg(short('A'), long("all"), long("no-ignore-removal"))]
    pub no_ignore_removal: bool,
    
    #[arg(long("no-all"), long("ignore-removal"))]
    pub ignore_removal: bool,

    #[arg(short('N'), long)]
    pub intent_to_add: bool,

    #[arg(long)]
    pub refresh: bool,
    
    #[arg(long)]
    pub ignore_errors: bool,
    
    #[arg(long)]
    pub ignore_missing: bool,

    #[arg(long)]
    pub no_warn_embedded_repo: bool,
    
    #[arg(long)]
    pub renormalize: bool,
    
    // This is quite not needed
    // #[arg(long)]
    // pub chmod: Option<>,

    #[arg(long)]
    pub pathspec_from_file: Option<PathBuf>,

    #[arg(long)]
    pub pathspec_file_null: bool
}