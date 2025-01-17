# bio-read

[![GitHub License](https://img.shields.io/github/license/PRO-2684/bio-read?logo=opensourceinitiative)](https://github.com/PRO-2684/bio-read/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/bio-read/release.yml?logo=githubactions)](https://github.com/PRO-2684/bio-read/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/bio-read?logo=githubactions)](https://github.com/PRO-2684/bio-read/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/bio-read/total?logo=github)](https://github.com/PRO-2684/bio-read/releases)
[![GitHub Downloads (all assets, latest release)](https://img.shields.io/github/downloads/PRO-2684/bio-read/latest/total?logo=github)](https://github.com/PRO-2684/bio-read/releases/latest)
[![Crates.io Version](https://img.shields.io/crates/v/bio-read?logo=rust)](https://crates.io/crates/bio-read)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/bio-read?logo=rust)](https://crates.io/crates/bio-read)

Bionic reading in terminal.

## 🚀 Installation

If you have `cargo-binstall`, you can install this tool by running:

```bash
$ cargo binstall bio-read
```

Otherwise, you can install it from source:

```bash
$ cargo install bio-read
```

## 📖 Usage

<details><summary>Click to see <code>bio-read</code> in action</summary>

> [!NOTE]
> The following asciinema recording is for demonstration purposes only. It may be outdated and may not reflect the latest version of `bio-read`.

[![asciicast](https://asciinema.org/a/1E6kkMHheWHnzUbKKcrnZ0lcl.svg)](https://asciinema.org/a/1E6kkMHheWHnzUbKKcrnZ0lcl)

</details>

```bash
$ br --help
Usage: br [<input>] [-f <fixation-point>] [-e <emphasize>] [-d <de-emphasize>]

Bionic reading in terminal.

Positional Arguments:
  input             the file to read from. Read from stdin if not specified.

Options:
  -f, --fixation-point
                    the fixation point. Should be in range [1, 5]. Default is 3.
  -e, --emphasize   customize how to emphasize the text. The emphasized text
                    will take the place of "{}". Example: --emphasize
                    "<em>{}</em>". Default to ansi bold.
  -d, --de-emphasize
                    customize how to de-emphasize the text. The de-emphasized
                    text will take the place of "{}". Example: --de-emphasize
                    "<de>{}</de>". Default to ansi dimmed.
  -h, --help        display usage information
```

For simple usage, run `br` with the file you want to read:

```bash
$ br file.txt
```

Alternatively, pipe the text you want to read into `br`:

```bash
$ cat file.txt | br
```

To set fixation points, use the `-f` flag, which is a shorthand for `--fixation-point`:

```bash
$ cat file.txt | br -f 1
```

## 📝 Notes

Although this tool aims to be as close to the [original bionic reading](https://reader.bionic-reading.com/) as possible, it is not exactly the same. Notably, the behavior differs when a word is too long, and it handles special characters differently. However, this tool is open-source, and guarantees linear time complexity and and constant memory usage.

On some terminals, you may need to configure emphasized text style to be "bold" for displaying ansi styles properly. Taking Windows Terminal for example, you'll need to set `profiles.defaults.intenseTextStyle` to `"bold"` or `"all"` in the `settings.json` file, or configure it in the settings UI.
