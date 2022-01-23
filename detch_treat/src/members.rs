use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Member {
    pub name: String,

    pub years: usize,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Member {
    pub fn new_now(name: String, years: usize) -> Member {
        let created_at: DateTime<Utc> = Utc::now();
        Member {
            name,
            years,
            created_at,
        }
    }
}

#[derive(Debug)]
pub struct AmountMember {
    pub amount: usize,

    pub member: Member,
}

impl AmountMember {
    pub fn new(amount: usize, member: Member) -> AmountMember {
        AmountMember { amount, member }
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
impl fmt::Display for AmountMember {
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
    todo!();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    let members = collect_members(&file)?;
    //members.iter().map(|m| m.years += years as usize);
    let _years = years;

    serde_json::to_writer(file, &members)?;
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

pub fn calc(journal_path: PathBuf, mut amount_all: usize, bias: Option<usize>) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let mut members = collect_members(&file)?;
    let mut _bias: usize = 0;

    if members.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Member list is empty!"));
    }

    if let Some(b) = bias {
        if b * members.len() > amount_all {
            return Err(Error::new(ErrorKind::InvalidInput, "Too much bias!"));
        } else {
            // 後にbiasをかける為に合計金額からマイナス
            amount_all -= b * members.len();
            // bias金額を上書き
            _bias = b;
        }
    }

    // 降順ソート
    members.sort_by(|a, b| b.years.cmp(&a.years));

    // 個人金額計算
    let years_sum = members.iter().fold(0, |sum, member| sum + member.years);
    let mut amount_members: Vec<AmountMember> = Vec::new();
    for m in members.clone() {
        // 割合計算
        let amount_percentile = (m.years as f64 / years_sum as f64) * (amount_all as f64);

        // 整数3桁に丸め
        let round_base = 100_f64;
        let amount_percentile_round = (amount_percentile / round_base).round() * round_base;

        // biasによって金額の調整
        amount_members.push(AmountMember::new(
            amount_percentile_round as usize + _bias,
            m,
        ));
    }

    // 差分計算
    let amount_delta = (amount_all as isize) + (_bias * members.len()) as isize
        - amount_members
            .iter()
            .fold(0, |sum, member| sum + member.amount) as isize;

    let mut order: u32 = 1;
    for member in amount_members {
        println!("{}: {}", order, member);
        order += 1;
    }

    println!("Surplus: {}", amount_delta);
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
