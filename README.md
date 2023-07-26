# Cached

A [Zellij](https://zellij.dev) plugin for persisiting `Vec<TabInfo>` and `PaneManifest` in the HD.

## Why?

TODO

## Installation

You'll need [rust](https://rustup.rs/) installed.

- `git clone git@github.com:nacho114/cached.git` TODO
- `cd cached`
- `cargo build --release`
- `mkdir -p ~/.config/zellij/plugins/`
- `mv target/wasm32-wasi/release/cached.wasm ~/.config/zellij/plugins/`

### Development

Make sure you have [rust](https://rustup.rs/) installed then run:

```sh
zellij action new-tab --layout ./dev.kdl
```
