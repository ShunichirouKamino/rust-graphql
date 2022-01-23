//! # todoアプリです。
//!

mod cli;
mod members;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs, InputIncrement, InputParticipant};
use members::Member;
use std::path::PathBuf;
use structopt::StructOpt;

/// # 関数に付与するコメントです。
///
/// メインエントリです。
fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add(InputParticipant { name, years }) => {
            members::add_participant(journal_file, Participant::new(name, years))
        }
        Increment(InputIncrement { years }) => members::increment(journal_file, years),
        Calc {} => members::calc(journal_file),
        List {} => members::out_list(journal_file),
    }?;
    Ok(())
}

/// # journalfile検索
///
/// - homeディレクトリを検索します
/// - homeディレクトリに、journalファイルの名称を付与して返却します
fn find_default_journal_file() -> Option<PathBuf> {
    let journal_file = ".rusty-journal.json";
    let pusher = |mut path: PathBuf| {
        path.push(journal_file);
        println!("{:?}", path);
        path
    };
    home::home_dir().map(pusher)
}
