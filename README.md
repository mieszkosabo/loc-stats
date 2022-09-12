# loc-stats

A lil' cli tool to get stats about your codebase, e.g. number of lines of code, lines of code per language, etc.

### Example usage:

```bash
loc-stats .
loc-stats /path/to/dir
```

## Installation

TODO:

## Roadmap

- [ ] add pretty output
- [ ] support omitting files listed in .gitignore
- [ ] add more stats, like number of files, percentages etc.
- [x] add json output format

## Testing

Run tests via a script instead of `cargo test` to clean up the test directory after running the tests.

```bash
./scripts/run_tests.sh
```
