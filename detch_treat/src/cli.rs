use std::path::PathBuf;
use structopt::StructOpt;
use validator::{Validate, ValidationError};

/// # Actionの直和型
///
/// コマンドライン引数を受け取り、それにともなう操作を直和型で定義します。
/// ドキュメンテーションコメントにより、help表示するコメントを記載することができます。
///
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write a perticipant to the journal file.
    Add(InputParticipant),
    /// Increment everyone's service of years.
    Increment(InputIncrement),
    /// Calculate the amount.
    Calc,
    /// Confirm the members list.
    List,
}

#[derive(Validate, StructOpt, Debug)]
pub struct InputParticipant {
    #[structopt()]
    pub name: String,

    #[structopt()]
    #[validate(range(min = 0, max = 100))]
    pub years: u8,
}

#[derive(Validate, StructOpt, Debug)]
pub struct InputIncrement {
    #[structopt()]
    pub years: i8,
}

/// # コマンドライン引数を読み取る構造体
///
/// Actionのラッパーとして起動し、subcommandで入力された型でActionをインスタンス化します。
#[derive(Debug, StructOpt)]
#[structopt(name = "to-do app", about = "A command line to-do app written in Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
