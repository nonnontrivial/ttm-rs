# ttm-rs

![Rust](https://github.com/nonnontrivial/ttm-rs/workflows/Rust/badge.svg)

> CLI to turn tuples into adjacency matrices

## Status

This is a hobby project in **early development**.

> Tested on `cargo 1.43.0-nightly`

### Features
- works with same args that [unix tsort](https://en.wikipedia.org/wiki/Tsort) accepts
- works with `.txt` tuple input

### Future
- works with `.md`, and `.json` file input
- works with `stdin` tuple input
- better formatting
- better tuple parsing

## Usage

Given a source file containing a directed graph represented as source->target
2-tuples, `ttm-rs` creates an adjacency matrix where `ij` is `1` to indicate
row `i` has column `j` as a target.

### Text file input

For example, `digraph.txt`

```txt
0 3
1 2
2 2
2 3
3 1
```

run using newline as tuple delimiter (default)

```shell
ttm-rs -f ./digraph.txt
```

prints to stdout

```shell
[[0, 0, 0, 1],
 [0, 0, 1, 0],
 [0, 0, 1, 1],
 [0, 1, 0, 0]]
```

## Install

### Cargo

```shell
cargo install ttm-rs
```
