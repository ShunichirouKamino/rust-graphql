use std::path::PathBuf;
use structopt::StructOpt;
use validator::Validate; //, ValidationError};

/// # This is an app for dividing money.
///
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write a memver to the journal file.
    Add(InputParticipant),
    /// Remove a member from the journal file.
    Remove(InputPosition),
    /// Increment everyone's service of years.
    Increment(InputIncrement),
    /// Calculate the amount.
    Calc(InputAmount),
    /// Confirm the members list.
    List,
}

#[derive(Validate, StructOpt, Debug)]
pub struct InputParticipant {
    #[structopt()]
    pub name: String,

    #[structopt()]
    #[validate(range(min = 0, max = 100))]
    pub years: usize,
}

#[derive(Validate, StructOpt, Debug)]
pub struct InputIncrement {
    #[structopt()]
    pub years: u8,
}

#[derive(Validate, StructOpt, Debug)]
pub struct InputPosition {
    #[structopt()]
    pub position: u8,
}

#[derive(Validate, StructOpt, Debug)]
pub struct InputAmount {
    #[structopt()]
    pub amount_all: usize,

    #[structopt()]
    pub bias: Option<usize>,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "detch treat app",
    about = "A command line detch treat app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
