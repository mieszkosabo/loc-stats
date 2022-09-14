# loc-stats

A lil' cli tool to get stats about your codebase, e.g. number of lines of code, lines of code per language, etc.

### Example usage:

```bash
loc-stats .
loc-stats --json --gitignore=false /path/to/dir
```

### Example output:

```

    __                      __        __
   / /___  _____      _____/ /_____ _/ /______
  / / __ \/ ___/_____/ ___/ __/ __ `/ __/ ___/
 / / /_/ / /__/_____(__  ) /_/ /_/ / /_(__  )
/_/\____/\___/     /____/\__/\__,_/\__/____/


Lines of code: 1913     Number of files: 11

Rust            1330    69.52%
JSON             479    25.03%
Markdown          72     3.76%
TOML              21     1.09%
Shell             10     0.52%
Other              1     0.05%
```

### Options

```bash
$ loc-stats --help

loc-stats 0.1.0

USAGE:
    loc-stats [OPTIONS] <PATH>

ARGS:
    <PATH>

OPTIONS:
        --gitignore <GITIGNORE>    Ignores files listed in .gitignore. Defaults to true [possible
                                   values: true, false]
    -h, --help                     Print help information
    -j, --json                     Gives the output in JSON format
    -V, --version                  Print version information
```

## Installation

1. Clone this repo.
2. Run `cargo build --release`
3. Add `target/release/loc-stats` to your path.

## Features

- Shows total number of lines of code, number of files and LOC grouped by language.
- Supports omitting files listed in .gitignore
- JSON output format with `--json` option

## Testing

Run tests via a script instead of `cargo test` to clean up the test directory after running the tests.

```bash
./scripts/run_tests.sh
```
