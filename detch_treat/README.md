# detch treat app

## Usage

- `$ cargo run -- help`

```sh
detch treat app 0.1.0
# This is an app for dividing money

USAGE:
    detch_treat [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-file <journal-file>    Use a different journal file

SUBCOMMANDS:
    add          Write a memver to the journal file
    calc         Calculate the amount
    help         Prints this message or the help of the given subcommand(s)
    increment    Increment everyone's service of years
    list         Confirm the members list
    remove       Remove a member from the journal file
```

- Check the target members.

  - `$ cargo run -- list`

- Add the member.

  - `$ cargo run -- add Name Years`

- Remove the member.

  - `$ cargo run -- add Serial`

- Calculate.
  - `$ cargo run -- calc amount [bias]`
