//! # todoアプリです。
//!

mod cli;
mod tasks;
use structopt::StructOpt;

/// # 関数に付与するコメントです。
///
/// メインエントリです。
fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
