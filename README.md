# i3auto-ws-icons-rs

`i3auto-ws-icons-rs` listens for i3 events and updates workspace names to show icons
for running programs. The icons can be configured by editing the `config.toml` configuration file.

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
exec_always --no-startup-id "$HOME/.cargo/bin/i3auto-ws-icons-rs"
```

## Configuration

After installing `i3auto-ws-icons-rust`, edit the [example configuration](https://raw.githubusercontent.com/mahur/i3auto-ws-icons-rust/master/examples/config.toml) to your liking.
The default location is `$XDG_CONFIG_HOME/i3auto-ws-icons-rust/config.toml`.

The configuration is driven through mapping of WM_CLASS of each window to the name of [Font Awesome](https://fontawesome.com/icons) icon.

`[icons]` table:
Key | Value | Default
----|-------------|----------
`WM_CLASS` of window | Icon name of font-awesome  | `*`

The default icon can be changed through by updating the `default_icon` property 

```
default_icon = '*'
```

The configuration comes with preset class names and icons for commonly used programs such as:

```toml
[[icons]]
alacritty = 'terminal'
kitty= 'terminal'
guake = 'terminal'
terminator = 'terminal'
firefox = 'firefox'
spotify = 'music'
slack = 'slack'
chromium = 'chrome'
code = 'file'
jetbrains-idea= 'code'
jetbrains-studio= 'code'
jetbrains-idea-ce= 'java'
"gimp-2.8"= 'image'
```
Any class names involving special characters such as hyphens and decimals should be enclosed in quotes as shown above.


## Debugging

Run `i3auto-ws-icons-rust` in a terminal to check the JSON it is outputting.  

## Acknowledgements

This project borrows some ideas and inspirations from following awesome open source projects:
* [justbuchanan/i3scripts](https://github.com/justbuchanan/i3scripts)
* [roosta/i3wsr](https://github.com/roosta/i3wsr)
* [JayceFayne/swayipc-rs](https://github.com/JayceFayne/swayipc-rs)
* [altdesktop/i3ipc-python](https://github.com/altdesktop/i3ipc-python)
* [BiagioFesta/i3-autolayout](https://github.com/BiagioFesta/i3-autolayout)
* [greshake/i3status-rust](https://github.com/greshake/i3status-rust)
