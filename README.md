# i3auto-ws-icons-rust

`i3auto-ws-icons-rs` listens for i3 events and updates workspace names to show icons
for running programs. The icons can be configured by editing the `config.toml` configuration file.

Inspired by [github.com/justbuchanan/i3scripts](https://github.com/justbuchanan/i3scripts)

## Getting Started

### Requirements for Compilation

The Rust compiler `rustc`, `cargo` package manager, C compiler `gcc` and `libssl-dev` packages are required to build the binary.

Compilation is only tested with very recent stable versions of `rustc`. If you use a distro with older Rust packages, consider using [rustup](https://rustup.rs/) to install a newer toolchain.

## Build and Install from Source

```shell
$ git clone https://github.com/madhur/i3auto-ws-icons-rust
$ cd i3auto-ws-icons-rust
$ cargo install --path . --locked
```

By default, this will install the binary to `~/.cargo/bin/i3status-rs`.
## Integrate it into i3/sway

The most preferred way is to run it with i3 configuration file and set it to start as startup as follows

```
exec_always --no-startup-id "$HOME/bin/i3auto-ws-icons-rs"
```

## Debugging

Run `i3auto-ws-icons-rust` in a terminal to check the JSON it is outputting.  