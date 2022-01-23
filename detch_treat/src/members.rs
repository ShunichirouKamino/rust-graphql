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

    pub years: usize,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Member {
    pub fn new(name: String, years: usize) -> Member {
        let created_at: DateTime<Utc> = Utc::now();
        Member {
            name,
            years,
            created_at,
        }
    }
}

#[derive(Debug)]
pub struct Amount_Member {
    pub amount: u8,

    pub member: Member,
}

impl Amount_Member {
    pub fn new(amount: u8, member: Member) -> Amount_Member {
        Amount_Member { amount, member }
    }
}

/// # Memberに対するフォーマット定義
///
impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<10} {:<10} [{}]", self.name, self.years, created_at)
    }
}

/// # Member_Amountに対するフォーマット定義
///
impl fmt::Display for Amount_Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self
            .member
            .created_at
            .with_timezone(&Local)
            .format("%F %H:%M");
        write!(
            f,
            "{:<10} {:<10} {:<10} [{}]",
            self.member.name, self.member.years, self.amount, created_at
        )
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
pub fn remove_member(journal_path: PathBuf, member_position: u8) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut members = collect_members(&file)?;

    if member_position == 0 || member_position > members.len().try_into().unwrap() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    members.remove((member_position - 1).try_into().unwrap());
    file.set_len(0)?;

    serde_json::to_writer(file, &members)?;
    Ok(())
}

/// # 年次の加算をおこなう
///
/// - jsonで定義されたファイルの読み込み
/// - 全員の年次を引数分加算
/// - 指定したtask_postionが0またはファイルサイズを超えた場合はエラー
///
pub fn increment(journal_path: PathBuf, years: u8) -> Result<()> {
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

/// # 現在登録されているメンバーリストの出力を行う
///
/// - jsonで定義されたファイルの読み込み
/// - Member一覧の出力
///
pub fn out_list(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let members = collect_members(&file)?;

    if members.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Member list is empty!"));
    } else {
        let mut order: u32 = 1;
        for member in members {
            println!("{}: {}", order, member);
            order += 1;
        }
    }
    Ok(())
}

pub fn calc(journal_path: PathBuf, amount_all: usize) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let mut members = collect_members(&file)?;

    if members.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Member list is empty!"));
    }

    members.sort_by(|a, b| b.years.cmp(&a.years));

    // パーセンタイル計算のために年次の合計値を計算
    // let years_sum: usize = 0;
    // for member in members {
    //     years_sum = years_sum + member.years;
    // }

    let years_sum = members.iter().fold(0, |sum, member| sum + member.years);
    for m in members {
        // 割合計算
        let percentile = (m.years as f64 / years_sum as f64) * (amount_all as f64);

        // 整数3桁で丸め
        let round_base = 100_f64;
        let percentile_round = (percentile / round_base).round() * round_base;
        println!("{}", percentile_round);
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
