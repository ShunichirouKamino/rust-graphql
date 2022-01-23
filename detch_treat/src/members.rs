use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub name: String,

    pub years: u8,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Member {
    pub fn new(name: String, years: u8) -> Member {
        let created_at: DateTime<Utc> = Utc::now();
        Member {
            name,
            years,
            created_at,
        }
    }
}

/// # Member
///
/// - Member
/// - フォーマッタfに対して、タスクテキストと変換後のcreated_atを定義
///
impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.name, created_at)
    }
}

/// # 参加者の追加をおこなう
///
/// - jsonで定義されたファイルの読み込み
/// - 参加者の追加
///
pub fn add_member(journal_path: PathBuf, member: Member) -> Result<()> {
    println!("{:?}", member);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_members(&file)?;
    tasks.push(member);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

/// # 年次の加算をおこなう
///
/// - jsonで定義されたファイルの読み込み
/// - 全員の年次を引数分加算
/// - 指定したtask_postionが0またはファイルサイズを超えた場合はエラー
///
pub fn increment(journal_path: PathBuf, years: i8) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_members(&file)?;

    // if task_position == 0 || task_position > tasks.len() {
    //     return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    // }
    // tasks.remove(task_position - 1);

    // file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn out_list(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_members(&file)?;

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

pub fn calc(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_members(&file)?;

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

/// # 参加者抽出処理
///
/// ファイルをインプットとし、参加者型のVecに変換して返却する。
///
/// - ファイルポインタを最初に巻き戻し
/// - ファイル内容の読み取り、MemberのVecに変換
/// - 再度ファイルポインタを最初に巻き戻し
///
fn collect_members(mut file: &File) -> Result<Vec<Member>> {
    // rewind the file before.
    file.seek(SeekFrom::Start(0))?;
    let members = match serde_json::from_reader(file) {
        Ok(members) => members,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(Error::from(e)),
    };
    // rewind the file after.
    file.seek(SeekFrom::Start(0))?;
    Ok(members)
}
