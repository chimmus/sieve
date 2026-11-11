# sieve

## Building
```bash
cargo build --release
cargo install --path .
```

## Usage
```
Usage: sieve [PATH]

Arguments:
  [PATH]  path

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Example:
```bash
sieve links.txt
```

Links in the file must have the protocol (e.g. `https://`)

Based off this [repo](https://github.com/ZorrentSystems/Website-Block-Checker)

This is a very basic version
