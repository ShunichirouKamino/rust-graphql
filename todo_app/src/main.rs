//! # todoアプリです。
//!

mod cli;
mod tasks;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

/// # 関数に付与するコメントです。
///
/// メインエントリです。
fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file.expect("Failed to find journal file");

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Faild to perform action")
}
