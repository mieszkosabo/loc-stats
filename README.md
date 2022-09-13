# loc-stats

A lil' cli tool to get stats about your codebase, e.g. number of lines of code, lines of code per language, etc.

### Example usage:

```bash
loc-stats .
loc-stats /path/to/dir
```

## Installation

TODO:

## Features

- supports omitting files listed in .gitignore
- json output format with --json option

## Testing

Run tests via a script instead of `cargo test` to clean up the test directory after running the tests.

```bash
./scripts/run_tests.sh
```
