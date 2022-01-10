use std::path::PathBuf;
use structopt::StructOpt;

/// # Actionの直和型
///
/// コマンドライン引数を受け取り、それにともなう操作を直和型で定義します。
#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        text: String,
    },
    Done {
        #[structopt()]
        position: usize,
    },
    List,
}

/// # コマンドライン引数を読み取る構造体
///
/// Actionのラッパーとして起動し、subcommandで入力された型でActionをインスタンス化します。
#[derive(Debug, StructOpt)]
#[structopt(name = "to-do app", about = "A command line to-do app written in Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
