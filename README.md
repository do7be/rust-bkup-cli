## Installation

```
cargo install bkup
```

## Usage

```
CLI to add a date to the end of the file name and makes a copy. e.g. filename.YYYYmmddHHMMSS

Usage: bkup [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -b, --bak       Create filename.bak
  -o, --original  Create filename.org
  -s, --simple    Create filename.YYYYmmdd
  -h, --help      Print help
  -V, --version   Print version
```

## Example

```
$ bkup example.txt
Created: example.txt.20230101010101
```

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/do7be/rust-bkup-cli/blob/main/LICENSE) file at the root of the repository.
