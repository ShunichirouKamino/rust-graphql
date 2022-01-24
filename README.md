# rust-sandbox

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

## GraphQL

`$ cargo init --bin graph-ql`

## Remote Container

**Admin**

- Open command palette
  - `Remote-Containes: Add Development Container Configuration Files...`
  - `Rust`
  - `buster`
    - [x] GitHub CLI

**Developers**

- Open a Remote Window
  - Add Proxy config to `.devcontainer/devcontainer.json`

```json
	"build": {
		"dockerfile": "Dockerfile",
		"args": {
			// Use the VARIANT arg to pick a Debian OS version: buster, bullseye
			// Use bullseye when on local on arm64/Apple Silicon.
			"VARIANT": "buster",
			"PROXY": "http://xxxx:8080" ←←
		}
	},
```
  - Reopen in Container
