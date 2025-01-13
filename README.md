# bio-read

[![GitHub License](https://img.shields.io/github/license/PRO-2684/bio-read?logo=opensourceinitiative)](https://github.com/PRO-2684/bio-read/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/bio-read/release.yml?branch=main&logo=githubactions)](https://github.com/PRO-2684/bio-read/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/bio-read?logo=githubactions)](https://github.com/PRO-2684/bio-read/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/bio-read/total?logo=github)](https://github.com/PRO-2684/bio-read/releases)
[![GitHub Downloads (all assets, latest release)](https://img.shields.io/github/downloads/PRO-2684/bio-read/latest/total?logo=github)](https://github.com/PRO-2684/bio-read/releases/latest)
[![Crates.io Version](https://img.shields.io/crates/v/bio-read?logo=rust)](https://crates.io/crates/bio-read)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/bio-read?logo=rust)](https://crates.io/crates/bio-read)

Bionic reading in terminal.

## 🚀 Installation

If you have `cargo-binstall`, you can install this tool by running:

```bash
cargo binstall bio-read
```

Otherwise, you can install it from source:

```bash
cargo install bio-read
```

## 📖 Usage

```bash
$ br --help
Usage: br [-f <fixation-point>] [-i <input>]

Bionic reading in terminal.

Options:
  -f, --fixation-point
                    the fixation point. Should be in range [1, 5]. Default is 3.
  -i, --input       the file to read from. Read from stdin if not specified.
  --help, help      display usage information
```

For simple usage, run `br` with the `-i` flag, which is a shorthand for `--input`:

```bash
$ br -i file.txt
```

Alternatively, pipe the text you want to read into `br`:

```bash
$ cat file.txt | br
```

To set fixation points, use the `-f` flag, which is a shorthand for `--fixation-point`:

```bash
$ cat file.txt | br -f 1
```
