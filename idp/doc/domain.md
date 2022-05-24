# domain for rust

# ドメインモデル

## 値オブジェクト


### 値オブジェクトとは？
[goでValueObject(値オブジェクト) を実装する](https://tech.isid.co.jp/entry/2021/12/17/go%E3%81%A7ValueObject%28%E5%80%A4%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%29_%E3%82%92%E5%AE%9F%E8%A3%85%E3%81%99%E3%82%8B)より、

> イミュータブルである  
> 不変条件が定義されており、条件を満たさない値では生成できない  
> 特定の属性で等価性が定義される  
> 値だけでなく、自身に属する機能を公開する  

を定義とします。
より抽象的な言い方をすると、プリミティブ型では持てない業務の振る舞いを型として表現したものです。

### 実装方針

`struct`にてフィールド変数を束縛し、`TryFrom`等のtraitや自前の`impl`を用いてメソッドを生やす。

- イミュータブルである
  - そもそもRustのオブジェクトは全てイミュータブルであり、値の変更は借用時を除いてできない。
  - `&mut`により借用した場合は値の変更は可能であるが、あくまで参照時のみであり、本体の値は変わらない。
- 不変条件が定義されており、条件を満たさない値では生成できない
  - `TryFrom`によるバリデーションの実装。
- 特定の属性で等価性が定義される  
  - `PartialEq`や`Eq`による拡張。
- 値だけでなく、自身に属する機能を公開する  
  - 計算ロジックや、他の値オブジェクトとの関わり等。
  - 業務的な実装が必要になるため、今回は省略。

### 実装

`src/domain/mail_address.rs`

- クラシックstructとして`String`から成る`MailAddress`型を定義。
- `TryFrom`により、コンストラクタを実装する。
  - このタイミングでRegexを確認し、`MailAddress`を構築できない場合にはエラーを出力する。
  - それ以外にも、値オブジェクト構築時のバリデート判定はこのタイミングで行う。
  - ここでの`Self`は`MailAddress`。
- structの各フィールドはmodule外からはprivateであるため、要素の取得用に`String`を拡張した`From`を`MailAddress`ジェネリクスで実装する。
  - ここでの`Self`は`String`。
  - 複数要素から成る値オブジェクトである場合は、素直にimplにgetter定義する。

```rs
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Serialize)]
pub struct MailAddress {
    mail_string: String,
}

// Constructs a value object following the regular expression of an email address.
impl TryFrom<String> for MailAddress {
    type Error = MyError;

    fn try_from(mail_string: String) -> Result<Self, Self::Error> {
        let regex = Regex::new(r#"^[a-zA-Z0-9_+-]+(.[a-zA-Z0-9_+-]+)*@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"#).unwrap();
        if regex.is_match(mail_string.as_str()) {
            Ok(Self { mail_string })
        } else {
            Err(my_error::MyError::InvalidValue)
        }
    }
}

impl MailAddress {
    pub fn of<T: Into<String>>(mail_string: T) -> Result<Self, MyError> {
        MailAddress::try_from(mail_string.into())
    }
}

/// MailAddress to String conversion process
impl From<MailAddress> for String {
    fn from(email: MailAddress) -> Self {
        email.mail_string
    }
}
```

### FAQ

#### **・構造体に文字列を持たせる場合、`String`と`&str`どっちがいいの？**

まず参照型であるため、構造体のフィールドに`&str`をそのまま用いることはできません。これは、以下のような利用からコンパイルエラーになります。

- `&str`が参照型である。
- 参照型をフィールドに持つ構造体は、借用チェッカー（ライフタイムチェッカー）によりコンパイルエラーになる。
- つまり、`MailAddress`のライフタイムと`mail_string: &str`のライフタイムが明示されていて、同一のライフタイムに属することを表現する必要が有る。

よって、仮に参照型を構造体のフィールドに持たせる場合は、以下のようにライフタイム参照ジェネリクスを用います。これにより、`MailAddress`と`mail_string`のライフタイムが同一の`'a`であることをコンパイラは理解します。

```rs
pub struct MailAddress<'a> {
    mail_string: &'a str,
}
```

しかし以下のように構造体のフィールドの借用元のライフタイムが、構造体本体のライフタイムより短いことはあり得るため、こういった場合利用側でコンパイルエラーにもなり得ます。

- `missing lifetime specifier
this function's return type contains a borrowed value, but there is no value for it to be borrowed from rustcE0106`

```rs
pub struct Person<'a> {
    name: &'a str,
}

fn get_user() -> Person {
    let name: String = "ichiro\njiro\nsaburo".to_string();
    for line in name.lines() {
        if line.starts_with("name: ") {
            return Person::new(&line);
        }
    } // <- ここでline: &strの寿命が切れるため返り値Personでコンパイルエラー
}
```

よって、逐次値を新規に`clone`してでも`String`で構築すべきです。

[(参考)Rust公式 - 構造体定義のライフタイム注釈](https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html#%E6%A7%8B%E9%80%A0%E4%BD%93%E5%AE%9A%E7%BE%A9%E3%81%AE%E3%83%A9%E3%82%A4%E3%83%95%E3%82%BF%E3%82%A4%E3%83%A0%E6%B3%A8%E9%87%88)

#### **・なぜtry_fromとofの2つのインスタンシエートメソッドがあるの？**

まずインスタンシエート時のモチベーションとして、以下の2つが有ります。

- `String`で保持する構造体に対して、`&str`でも`String`でも引数として受け入れたい。
- 失敗する可能性のある型変換は、`trait`に対して明示的に`TryFrom`を実装したい。
  - 特に値オブジェクトの場合は、インスタンシエートに対して条件を付けることが多く、`TryFrom`が適している。


上記を満たすために、本来`Into<String>`によるトレイト境界を用いて、以下のような実装がしたいです。
- `mail_string`は`Into<String>`であることで、`try_from`の引数は`String`でも`&str`でも良いです。
- 構造体を構築するタイミングで`into`を実施し、`&str`の場合は`String`に変換され、`String`の場合は変換は行われません。

```rs
impl<S> TryFrom<S> for MailAddress
where
    S: Into<String>,
{
    type Error = MyError;

    fn try_from(mail_string: S) -> Result<Self, Self::Error> {
        let regex = Regex::new(r#"^[a-zA-Z0-9_+-]+(.[a-zA-Z0-9_+-]+)*@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"#).unwrap();
        if regex.is_match(mail_string.as_str()) {
            Ok(Self { mail_string.into() })
        } else {
            Err(my_error::MyError::InvalidValue)
        }
    }
}
```

（参考）下記が`&str`に対して実装されているため、型推論が可能な場合のみ`into`は上記の動作となります。ここあまり理解できてません。

```rs
// https://doc.rust-lang.org/src/core/convert/mod.rs.html#539-552

impl<T, U> const Into<U> for T
where
    U: ~const From<T>,
{
    /// Calls `U::from(self)`.
    ///
    /// That is, this conversion is whatever the implementation of
    /// <code>[From]&lt;T&gt; for U</code> chooses to do.
    fn into(self) -> U {
        U::from(self)
    }
}

// https://doc.rust-lang.org/src/core/convert/mod.rs.html#557-562

impl<T> const From<T> for T {
    /// Returns the argument unchanged.
    fn from(t: T) -> T {
        t
    }
}
```
しかし、上記実装は、`core::convert`の`TryFrom`実装と競合します。

```rs
// https://doc.rust-lang.org/beta/src/core/convert/mod.rs.html#598-607
impl<T, U> const TryFrom<U> for T
where
    U: ~const Into<T>,
{
    type Error = Infallible;

    fn try_from(value: U) -> Result<Self, Self::Error> {
        Ok(U::into(value))
    }
}
```

よって、`of`により`TryFrom`をラップすることで、`&str`でも`String`でも受け取ることができ、かつバリデーション判定を行うコンストラクタを構築できます。


## エンティティ

エンティティ

`src/domain/entity/user.rs`

```rs
use crate::domain::mail_address::MailAddress;

/// Entities consist of classic structures.
/// Represents a mutable object.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct User {
    pub email: MailAddress,
}

// Factory that instantiates from field values
impl User {
    pub fn of(email: MailAddress) -> Self {
        Self { email }
    }


```