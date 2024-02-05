use clap::Parser;
use commands::Git;

mod commands;

fn main()  {
    let git_command = Git::parse();
    print!("{:?}", git_command);
}
