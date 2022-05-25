# domain for rust

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [domain for rust](#domain-for-rust)
  - [値オブジェクト](#値オブジェクト)
    - [値オブジェクトとは？](#値オブジェクトとは)
    - [実装方針](#実装方針)
    - [実装](#実装)
    - [FAQ](#faq)
      - [**Q. 等価性の定義はどのように行っているの？**](#q-等価性の定義はどのように行っているの)
      - [**Q. 構造体に文字列を持たせる場合、`String`と`&str`どっちがいいの？**](#q-構造体に文字列を持たせる場合-stringとstrどっちがいいの)
      - [**Q. なぜ try_from と of の 2 つのインスタンシエートメソッドがあるの？**](#q-なぜ-try_from-と-of-の-2-つのインスタンシエートメソッドがあるの)
      - [**Q. MyError って何？**](#q-myerror-って何)
  - [エンティティ](#エンティティ)

<!-- /code_chunk_output -->

## 値オブジェクト

### 値オブジェクトとは？

[go で ValueObject(値オブジェクト) を実装する](https://tech.isid.co.jp/entry/2021/12/17/go%E3%81%A7ValueObject%28%E5%80%A4%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%29_%E3%82%92%E5%AE%9F%E8%A3%85%E3%81%99%E3%82%8B)より、

> イミュータブルである  
> 不変条件が定義されており、条件を満たさない値では生成できない  
> 特定の属性で等価性が定義される  
> 値だけでなく、自身に属する機能を公開する

を定義とします。
より抽象的な言い方をすると、プリミティブ型では持てない業務の振る舞いを型として表現したものです。

### 実装方針

`struct`にてフィールド変数を束縛し、`TryFrom`等の trait や自前の`impl`を用いてメソッドを生やす。

- イミュータブルである
  - そもそも Rust のオブジェクトは全てイミュータブルであり、値の変更は借用時を除いてできない。
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

- クラシック struct として`String`から成る`MailAddress`型を定義。
- `TryFrom`により、コンストラクタを実装する。
  - このタイミングで Regex を確認し、`MailAddress`を構築できない場合にはエラーを出力する。
  - それ以外にも、値オブジェクト構築時のバリデート判定はこのタイミングで行う。
  - ここでの`Self`は`MailAddress`。
- struct の各フィールドは module 外からは private であるため、要素の取得用に`String`を拡張した`From`を`MailAddress`ジェネリクスで実装する。
  - ここでの`Self`は`String`。
  - 複数要素から成る値オブジェクトである場合は、素直に impl に getter 定義する。

```rust
use regex::Regex;
use serde::Serialize;
use std::convert::TryFrom;

use crate::error::my_error::{self, MyError};

/// Value objects are tuple structures because they are one primitive-based.
/// Uniquely identifies a user.
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Serialize)]
pub struct MailAddress {
    mail_string: String,
}

// Constructs a value object following the regular expression of an email address.
impl TryFrom<String> for MailAddress {
    type Error = MyError;

    fn try_from(mail_string: String) -> my_error::Result<Self> {
        let regex = Regex::new(r#"^[a-zA-Z0-9_+-]+(.[a-zA-Z0-9_+-]+)*@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"#).unwrap();
        if regex.is_match(mail_string.as_str()) {
            Ok(Self { mail_string })
        } else {
            Err(my_error::MyError::InvalidValue)
        }
    }
}

impl MailAddress {
    pub fn of<T: Into<String>>(mail_string: T) -> my_error::Result<Self> {
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

#### **Q. 等価性の定義はどのように行っているの？**

`#[derive]`属性により、`PartialEq`トレイト及び`Eq`トレイトを追加しています。

- `PartialEq`
  - 同値の中でも推移律と対象律を満たす場合に付与します。
- `Eq`
  - `PartialEq`に加えて反射律を満たす場合に付与します。
  - `PartialEq`をスーパートレイトとして持ちます。
    - `PartialEq`に加えて何かを実装しているわけではなく、あくまで反射律を満たしませんよという点を実装者に伝えています。

```rust
// https://doc.rust-lang.org/src/core/cmp.rs.html#218-233
pub trait PartialEq<Rhs: ?Sized = Self> {
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn eq(&self, other: &Rhs) -> bool;

    /// This method tests for `!=`.
    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[default_method_body_is_const]
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

// https://doc.rust-lang.org/src/core/cmp.rs.html#286-299
pub trait Eq: PartialEq<Self> {
    #[doc(hidden)]
    #[no_coverage] // rust-lang/rust#84605
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn assert_receiver_is_total_eq(&self) {}
}
```

`MailAddress`は`String`から成るオブジェクトで有り、これは反射率も満たしているため、`Eq`トレイトも付与しています。例えばフィールドに`float`のような反射率を満たさないフィールドが存在する場合、`Eq`の付与はできません。（コンパイルエラーになります）

（参考）同値について  
離散数学において、推移律・対象律・反射律を満たすものを`同値`と呼びます。ある集合 A に属する(x,y,z)を考えます。

- 対象律は、x=y ならば、y=x である事を指します。
- 推移律は、x=y かつ y=z ならば、z=x である事を指します。
- 反射律は、全ての要素 n において、n=n であることを指します。

例えば String という集合を考えた際に、String に(x,y,z)が存在したとします。（※x,y,z はただの記号で有り、同様の値が格納されている可能性も有ります。いわば変数名です。）

- 集合内の任意の値、x=y の場合、y=x が満たされるため、対象律は満たされます。
- 集合内の任意の値、x=y かつ y=z の場合、z=x でもあるため、推移律は満たされます。
- ある要素 x について、x=x です。これは、y,z についても同様のことが言えるため、反射率は満たされます。

例えば float の集合を考えた際に、float に(a,b,c,NaN)が存在したとします。（※a,b,c はただの記号ですが、NaN は NaN を表します。）

- 集合内の任意の値、任意の a=b の場合、b=a が満たされるため、対象律は満たされます。
- 集合内の任意の値、a=b かつ c=a の場合、a=c でもあるため、推移律は満たされます。
- a=a,b=b,c=c は満たされますが、要素 NaN については、浮動小数点誤差により NaN!=NaN です。つまり、すべての要素について同値ではないため、反射律は満たされません。

NaN は、浮動小数点計算による異常値（数字で表されない値）のことで、例えば`0.0/0.0`や`無限大-無限大`の際に登場します。Rust においては、以下で実装されています。

```rust
// https://doc.rust-lang.org/src/core/num/f64.rs.html#421
pub const NAN: f64 = 0.0_f64 / 0.0_f64;
```

#### **Q. 構造体に文字列を持たせる場合、`String`と`&str`どっちがいいの？**

まず参照型であるため、構造体のフィールドに`&str`をそのまま用いることはできません。これは、以下のような利用からコンパイルエラーになります。

- `&str`が参照型である。
- 参照型をフィールドに持つ構造体は、借用チェッカー（ライフタイムチェッカー）によりコンパイルエラーになる。
- つまり、`MailAddress`のライフタイムと`mail_string: &str`のライフタイムが明示されていて、同一のライフタイムに属することを表現する必要が有る。

よって、仮に参照型を構造体のフィールドに持たせる場合は、以下のようにライフタイム参照ジェネリクスを用います。これにより、`MailAddress`と`mail_string`のライフタイムが同一の`'a`であることをコンパイラは理解します。

```rust
pub struct MailAddress<'a> {
    mail_string: &'a str,
}
```

しかし以下のように構造体のフィールドの借用元のライフタイムが、構造体本体のライフタイムより短いことはあり得るため、こういった場合利用側でコンパイルエラーにもなり得ます。

- `missing lifetime specifier this function's return type contains a borrowed value, but there is no value for it to be borrowed from rustcE0106`

```rust
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

[(参考)Rust 公式 - 構造体定義のライフタイム注釈](https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html#%E6%A7%8B%E9%80%A0%E4%BD%93%E5%AE%9A%E7%BE%A9%E3%81%AE%E3%83%A9%E3%82%A4%E3%83%95%E3%82%BF%E3%82%A4%E3%83%A0%E6%B3%A8%E9%87%88)

#### **Q. なぜ try_from と of の 2 つのインスタンシエートメソッドがあるの？**

まずインスタンシエート時のモチベーションとして、以下の 2 つが有ります。

- `String`で保持する構造体に対して、`&str`でも`String`でも引数として受け入れたい。
- 失敗する可能性のある型変換は、`trait`に対して明示的に`TryFrom`を実装したい。
  - 特に値オブジェクトの場合は、インスタンシエートに対して条件を付けることが多く、`TryFrom`が適している。

上記を満たすために、本来`Into<String>`によるトレイト境界（ジェネリックに対して型を付け、そのルールの有効範囲を指定）を用いて、以下のような実装がしたいです。

- `mail_string`は`Into<String>`であることで、`try_from`の引数は`String`でも`&str`でも良いです。
- 構造体を構築するタイミングで`into`を実施し、`&str`の場合は`String`に変換され、`String`の場合は変換は行われません。

```rust
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

しかし、上記実装は、`core::convert`の`TryFrom`実装と競合します。

```rust
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

よって、`of`により`TryFrom`をラップすることで、`&str`でも`String`でも受け取ることができ、かつバリデーション判定を行うコンストラクタを構築しています。

（参考）`into()`により、`&str`が`String`に変換可能な理由は、下記が`&str`に対して実装されているためです。型推論が可能な場合のみ`into`は上記の動作となります。ここあまり理解できてません。

```rust
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

#### **Q. MyError って何？**

`TryFrom` の`trait`は、以下のようになっています。これは、`Error`として何らかの型を定義し、`try_from`の`Result`型に適用することを意味しています。今回はこれを利用し、バリデーション判定を行っています。

```rust
// https://doc.rust-lang.org/stable/src/core/convert/mod.rs.html#469-477
#[rustc_diagnostic_item = "TryFrom"]
#[stable(feature = "try_from", since = "1.34.0")]
pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    #[stable(feature = "try_from", since = "1.34.0")]
    type Error;

    /// Performs the conversion.
    #[stable(feature = "try_from", since = "1.34.0")]
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```

例えば、0 以上の数値からしか生成できない`TryFrom`の実装を簡単に行うと、以下のようになります。
`Error`には自前の Error を定義せず、あくまで&str として返却されます。

```rust
impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}
```

今回の実装では、自前で`MyError`型を定義しています。

```rust
impl TryFrom<String> for MailAddress {
    type Error = MyError;

    fn try_from(mail_string: String) -> my_error::Result<Self> {
        let regex = Regex::new(r#"^[a-zA-Z0-9_+-]+(.[a-zA-Z0-9_+-]+)*@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"#).unwrap();
        if regex.is_match(mail_string.as_str()) {
            Ok(Self { mail_string })
        } else {
            Err(my_error::MyError::InvalidValue)
        }
    }
}
```

- my_error.rs

```rust
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum MyError {
    Decode,
    Encode,
    InvalidValue,
}

impl Error for MyError {}

pub type Result<T, E = MyError> = std::result::Result<T, E>;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::InvalidValue => f.write_str("Invalid Value Error"),
            MyError::Decode => f.write_str("Decode Error"),
            MyError::Encode => f.write_str("Encode Error"),
        }
    }
}
```

- MyError として、どういったエラーが発生するかの種類を列挙で保持します。
  - 特にドメイン実装時に必要なのは、`InvalidValue`です。
- `impl Error for MyError {}`にて`MyError`に`Error`トレイトを実装しています。これは、`Debug`と`Display`の実装を強要するためです。
  - これまでは、`description`メソッドをオーバライドすることで`Error`時のコンソールログを実装していたようですが、現在は`Display`と`Debug`をスーパートレイトとしているようで、それらを用いてコンソールログを実装してねという方針のようです。
- `Result`は、`std::result`に対して`MyError`を事前に投入しておくシンタックスシュガーです。
- `Debug`については`derive`アトリビュートで実装していますが、`Display`については自前で実装しています。これは、`Enum` の列挙定数がそのままコンソールに出力されてしまうためです。
  - 今後のことを考えると、`Enum`列挙定数に`String`等を持たせ、その値も含めて出力することを考えると、`Display`のみ拡張しておくほうが良いと考えました。

（参考）std の error 実装

````rust
// https://doc.rust-lang.org/src/std/error.rs.html#55-160
pub trait Error: Debug + Display {
    #[stable(feature = "error_source", since = "1.30.0")]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    /// Gets the `TypeId` of `self`.
    #[doc(hidden)]
    #[unstable(
        feature = "error_type_id",
        reason = "this is memory-unsafe to override in user code",
        issue = "60784"
    )]
    fn type_id(&self, _: private::Internal) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }

    /// Returns a stack backtrace, if available, of where this error occurred.
    ///
    /// This function allows inspecting the location, in code, of where an error
    /// happened. The returned `Backtrace` contains information about the stack
    /// trace of the OS thread of execution of where the error originated from.
    ///
    /// Note that not all errors contain a `Backtrace`. Also note that a
    /// `Backtrace` may actually be empty. For more information consult the
    /// `Backtrace` type itself.
    #[unstable(feature = "backtrace", issue = "53487")]
    fn backtrace(&self) -> Option<&Backtrace> {
        None
    }

    /// ```
    /// if let Err(e) = "xc".parse::<u32>() {
    ///     // Print `e` itself, no need for description().
    ///     eprintln!("Error: {e}");
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(since = "1.42.0", reason = "use the Display impl or to_string()")]
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_deprecated(
        since = "1.33.0",
        reason = "replaced by Error::source, which can support downcasting"
    )]
    #[allow(missing_docs)]
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
````

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
