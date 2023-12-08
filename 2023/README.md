# AoC 2023

Based off of [fspoettel's](https://github.com/fspoettel/advent-of-code-rust) template but not cloned as a template through Github's UI.  It's a hack job on my end, I like his setup.

```sh
cargo scaffold <day>
cargo solve <day>
cargo test --bin 01

bacon test -- --bin 01

# or something like if bacon is annoying
watchexec -c -- cargo test --bin 01 -- --nocapture

# or something to focus on a test
watchexec -c -- cargo test --bin 01 -- --nocapture test_name_here
```

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module "src/bin/01.rs"
# Created empty input file "src/inputs/01.txt"
# Created empty example file "src/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```
