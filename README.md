# ttm-rs

![Rust](https://github.com/nonnontrivial/ttm-rs/workflows/Rust/badge.svg)

## Purpose

This is a CLI that turns tuples in a `.txt` file into _adjacency matrices_.

> **Status**: This is a hobby project in **early development**.

### Known Limitations

The following desirable features are not yet implemented.

- works with same args that [unix tsort](https://en.wikipedia.org/wiki/Tsort) accepts
- works with `.md`, and `.json` file input
- works with `stdin` tuple input
- better formatting
- better tuple parsing

## Usage

Given a source file containing a directed graph represented as newline-separated
source-target 2-tuples, `ttm-rs` creates an adjacency matrix where `ij` is `1`
when row `i` has column `j` as a target.

### Text File Input

For example, `digraph.txt`

```txt
0 3
1 2
2 2
2 3
3 1
```

passed under `-f` (file) flag

```shell
ttm-rs -f ./digraph.txt
```

prints to stdout:

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
