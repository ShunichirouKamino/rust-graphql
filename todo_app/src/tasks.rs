use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

/// # タスクの追加をおこなう
///
/// - jsonで定義されたファイルの読み込み
/// - タスクの追加
///
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

/// # タスクの完了（削除）を行う
///
/// - jsonで定義されたファイルの読み込み
/// - タスクの完了（削除）
/// - 指定したtask_postionが0またはファイルサイズを超えた場合はエラー
///
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // let tasks = match serde_json::from_reader(file) {
    //     Ok(tasks) => tasks,
    //     Err(e) if e.is_eof() => Vec::new(),
    //     Err(e) => Err(e)?,
    // };

    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

/// # タスク抽出処理
///
/// ファイルをインプットとし、タスク定義に変換して返却する。
///
/// - ファイルポインタを最初に巻き戻し
/// - ファイル内容の読み取り、Taskのベクタに変換
/// - 再度ファイルポインタを最初に巻き戻し
///
fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // rewind the file before.
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    // rewind the file after.
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}
