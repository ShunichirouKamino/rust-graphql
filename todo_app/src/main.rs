//! # todoアプリです。
//!

mod cli;
use structopt::StructOpt;

/// # 関数に付与するコメントです。
///
/// メインエントリです。
fn main() {
    cli::CommandLineArgs::from_args();
}
