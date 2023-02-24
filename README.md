<div align='center'>

# Sinc

<samp>

**A cross-platform dotfiles manager**

</samp>

</div>

## 🚧 WIP

This repository is still development. Use at your own lisk.

## 📦 Installation

```shell
$ cargo install sinc
```

## ⚡️ Getting started

### Config file

Sing looks for the config file in the following location:

- `$SINC_DIR/sinc/sinc.toml`
- `$XDG_CONFIG_HOME/sinc/sinc.toml`
- `$HOME/.config/sinc/sinc.toml`

### Minimal config

The following is the minimal config:

```toml
[default]
dir = "~/.dotfiles"
sync_type = "symlink"

[[dotfiles]]
path = "sinc"
target = "~/.config/sinc"
```

If you run the `sinc` command in this state, a symbolic link is created from `~/.dotfiles/sinc` to `~/.config/sinc`. \
In other words, the config files mentioned earlier have been moved to `~/.dotfiles/sinc` and a symbolic link has been created to the original location.

<div align='center'>

```haskell
(dir + path)         (sync_type)            (target)
 │                    │                    │
 V                    V                    V
~/.dotfiles/sinc  <<== symbolic link ==>>  ~/.config/sinc
```

</div>

Now, some of you may have noticed a problem here. \
Yes, this config does not support applications that have different location for their config file depending on the OS.

To solve this, you can use `match(os)` instead of specifying the value directly.

```toml
[default]
dir = "~/.dotfiles"
sync_type."match(os)" = { default = "symlink", windows = "junction" }

[[dotfiles]]
path = "nvim"
  [dotfiles.target."match(os)"]
  default = "~/.config/bat"
  windows = "~/AppData/Roaming/bat"

[[dotfiles]]
path = ".gitconfig"
target = "~/.gitconfig"
sync_type."match(os)" = { default = "symlink", windows = "hardlink" }
```

## 🔧 Configration

### `config.default` : Table

| Value | Type |
| - | - |
|`dir`|`String`|
|`sync_type`|`SyncType`|

### `config.dotfiles` : Array

| Value | Type |
| - | - |
|`dir`|`Option<String>`|
|`sync_type`|`Option<SyncType>`|
|`target`|`String`|
|`path`|`String`|
|`enable`|`Option<Boolean>`|

### `SyncType` : String

| Value | Description |
| - | - |
|`"symlink"`|If you are using Windows, you need administrator rights|
|`"junction"`|Windows only / used to link to folder|
|`"hardlink"`|Behave like the original file / used to link to file|
|`"copy"`|Just copy|

### `config.dotfiles.Value.match(...)` : Table

| Value | Key | Example |
| - | - | - |
|`match(os)`|<samp>[os_info::Type](https://docs.rs/os_info/latest/os_info/enum.Type.html)</samp> (lower case)|`arch linux`|
|`match(os_type)`|<samp>[std::env::consts::OS](https://doc.rust-lang.org/std/env/consts/constant.OS.html)</samp>|`linux`|
|`match(os_family)`|<samp>[std::env::consts::FAMILY](https://doc.rust-lang.org/std/env/consts/constant.FAMILY.html)</samp>|`unix`|

## 📃 License

[MIT](./LICENSE)
