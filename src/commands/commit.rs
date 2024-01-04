use std::path::PathBuf;

use clap::{arg, Args, ValueEnum};

type CommitHash = String;

#[derive(Debug, Clone, Args)]
pub struct CommitArgs {
    #[arg(long, conflicts_with("patch"), conflicts_with("all"))]
    pub interactive: bool,

    #[arg(short, long, conflicts_with("interactive"), conflicts_with("all"))]
    pub patch: bool,

    #[arg(short, long, conflicts_with("interactive"), conflicts_with("patch"))]
    pub all: bool,

    #[arg(long)]
    pub amend: bool,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(short('F'), long)]
    pub file: Option<PathBuf>,

    #[arg(
        short('C'),
        long,
        value_name("COMMIT"),
        conflicts_with("reedit_message"),
        conflicts_with("squash")
    )]
    pub reuse_message: Option<CommitHash>,

    #[arg(
        short('c'),
        long,
        value_name("COMMIT"),
        conflicts_with("reuse_message"),
        conflicts_with("squash")
    )]
    pub reedit_message: Option<CommitHash>,

    #[arg(long)]
    pub reset_author: bool,

    #[arg(long)]
    pub author: Option<String>,

    #[arg(long)]
    pub date: Option<String>,

    #[arg(short, long)]
    pub template: Option<String>,

    #[arg(long)]
    pub allow_empty: bool,
    
    #[arg(long)]
    pub allow_empty_message: bool,

    #[arg(long, value_name("MODE"))]
    pub cleanup: Option<CleanupOptions>,

    #[arg(short, long)]
    pub edit: bool,

    #[arg(long)]
    pub no_edit: bool,

    #[arg(short, long)]
    pub include: bool,
    
    #[arg(short, long)]
    pub only: bool,

    // we need here smth like --fixup=[(amend|reword):]<commit>
    #[arg(long, num_args=1, value_name("[amend|reword: ]<COMMIT>"))]
    pub fixup: Option<FixupArgs>, // can it be done better with Clapy???

    #[arg(
        long,
        value_name("COMMIT"),
        conflicts_with("reuse_message"),
        conflicts_with("reedit_message")  
    )]
    pub squash: Option<CommitHash>,

    #[arg(long)]
    pub pathspec_from_file: Option<PathBuf>,

    #[arg(long)]
    pub pathspec_file_null: bool
}

#[derive(Debug, Clone)]
 pub struct  FixupArgs {
    pub command: Option<FixupOptions>,
    pub commit: String
}

impl std::str::FromStr for FixupArgs {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split(":").collect();

        match parts.len() {
            1 => Ok(FixupArgs {
                command: None,
                commit: parts[0].to_string()
            }),
            2 => Ok(FixupArgs{
                command: Some(parts[0].parse::<FixupOptions>()?),
                commit: parts[1].to_string(),
            }),
            _ => Err(String::from("Invalid number of arguments"))
        }
    }
}

#[derive(Debug, Clone)]
pub enum FixupOptions {
    Amend,
    Reword
}


impl std::str::FromStr for FixupOptions {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "amend" => Ok(FixupOptions::Amend),
            "reword" => Ok(FixupOptions::Reword),
            _ => Err(String::from("[amend | reword]"))
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CleanupOptions {
    Default = 0,
    Strip,
    Whitespace,
    Verbatim,
    Scissors,
}