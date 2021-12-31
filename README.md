# rust-graphql

## Settings

- [install C++ Build Tools](https://visualstudio.microsoft.com/ja/visual-cpp-build-tools/)
  - or [Microsoft Visual Studio](https://visualstudio.microsoft.com/ja/downloads/)
- [install Rust](https://www.rust-lang.org/tools/install)
- install `rust-analyzer` by Visual Studio MarketPlace
- setup rust-analyzer

```bash
rustup component add rust-src
rustup component add rust-analysis
rustup component add rls
```

## Hello World!

- `$ cargo new hello_world`
- `$ cd hello_world/src`
- `$ rustc main.rs`
- `$ ./main`

```bash
Hello, world!
```

- `$ cargo run`
