# Changelog

All notable changes to this project will be documented in this file.

## [0.6.5] - 2024-06-12

### Changed

* Better error messages for plugin communication
* Evaluation of plugin path. Use `PATH` environment variable to find the plugin
  executable.

## [0.6.4] - 2024-06-10

### Changed

* Switch to vendored openssl. The nuts container brings its own `libcrypto` and
  does not rely on an shared `libcrypto` anymore.
* Compact debug/trace messages. Compact messages from the nuts-tool/plugin
  communication by not logging whole byte arrays. Instead only log number by
  bytes.
* Debug level of plugin. Adapt verbose level passed to the plugin to the
  verbose level of the nuts tool. There is no need to enable tracing for the
  plugin if nuts tool does not enable it.

## [0.6.3] - 2024-06-03

### Changed

* Strip all binaries in release mode
* Work on test coverage

## [0.6.2] - 2024-05-23

### Changed

* Update dependencies:

  - `anyhow` 1.0.79 -> 1.0.86
  - `chrono` 0.4.31 -> 0.4.38
  - `clap` 4.4.18 -> 4.5.4
  - `env_logger` 0.10.1 -> 0.10.2
  - `getrandom` 0.2.12 -> 0.2.15
  - `log` 0.4.20 -> 0.4.21
  - `openssl` 0.10.62 -> 0.10.64
  - `openssl-sys` 0.9.98 -> 0.9.102
  - `proc-macro2` 1.0.76 -> 1.0.83
  - `quote` 1.0.35 -> 1.0.36
  - `serde` 1.0.197 -> 1.0.202
  - `syn` 2.0.48 -> 2.0.65
  - `tempfile` 3.9.0 -> 3.10.1
  - `thiserror` 1.0.56 -> 1.0.61
  - `toml` 0.8.11 -> 0.8.13

* The data exchange protocol between plugin and `nuts` tool has now a revision,
  which is now `1`. The revious protocol revision is `0`.

## [0.6.1] - 2024-05-21

### Fixed

* Fix lints found by `rust-clippy`.
* Remove unnecessary `unwrap()` calls.

## [0.6.0] - 2024-05-16

### Added

* `nuts-tool`: Introduction of a plugin system. Now the `nuts` tool can handle
               various backends without recompile it.
* `nuts-tool-api`: The new crate contains code which makes it easier to
                   implement a `nuts` tool plugin.
* `nuts-directory`: Is additionally implemented as a `nuts` tool plugin. Can be
                    enabled with the `plugin` feature.

### Changed

* `nuts-backend`: Some refactoring useful for implementing the `nuts` tool
                  plugin.

## [0.5.0] - 2024-03-05

### Changed

* `nuts-backend`: Remove support for a null-id, which does not fit into the
                  Rust world. Therefore the `BlockId` trait was removed.
* `nuts-memory`: Adapt `nuts-backend` changes.
* `nuts-directory`: Adapt `nuts-backend` changes.
* `nuts-archive`:
  - Refactor the node-tree because of removed null-id in `nuts-backend`.
  - Switch to a new revision: 2
  - **Old archives with revision 1 cannot be opened anymore.**

## [0.4.3] - 2024-01-22

### Changed

Update dependencies:

* `clap` 3.2.23 -> 4.4.18

### Fixed

* Replace `tempdir` crate by `tempfile`. There was a security advisory in
  `remove_dir_all` crate.

## [0.4.2] - 2024-01-18

### Added

* `nuts-bytes`: Implement `PutBytes` for `&mut Vec<u8>`

## [0.4.1] - 2024-01-15

### Changed

Update dependencies:

* `anyhow` 1.0.75 -> 1.0.79
* `colored` 2.0.4 -> 2.1.0
* `env_logger` 0.9.3 -> 0.10.1
* `getrandom` 0.2.10 -> 0.2.100.2.12
* `home` 0.5.5 -> 0.5.9
* `openssl` 0.10.57 -> 0.10.62
* `openssl-sys` 0.9.93 -> 0.9.98
* `rpassword` 7.2.0 -> 7.3.1
* `thiserror` 1.0.50 -> 1.0.56

## [0.4.0] - 2024-01-10

Huge refactoring release. Put all `nuts-*` projects into a Cargo workspace
project.

## nuts-archive

### 0.2.4 - 2023-12-05

#### Changed

- Removed Serde dependency
- Update dependencies:
  * `nuts-container` 0.1.1 -> 0.2.2
  * `nuts-bytes` 0.1.1 -> 0.2.2
  * `nuts-directory` 0.3.1 -> 0.3.2

### 0.2.3 - 2023-11-10

#### Added

- `Archive::lookup()` to search for an entry

### 0.2.2 - 2023-11-07

#### Added

- Put timestamps into entries

#### Changed

- Remove `Mode` from public interface

### 0.2.1 - 2023-11-03

#### Changed

- Move `container::BufContainer` to `pager::Pager`
- Correct handling of `Error::InvalidBlockSize` error
- Put id into `Error::InvalidType` error

### 0.2.0 - 2023-11-03

#### Added

- The `Mode` type encodes the type and access rights and of an entry.

#### Changed

- Refactoring of `Entry` & `EntryMut` which now includes the `Mode`.

### 0.1.1 - 2023-10-27

#### Changed

- Refactoring of the header of the archive. The header block is guarded by a
  magic value.

### 0.1.0 - 2023-10-25

Initial public release.

## nuts-bytes

### 0.2.2 - 2023-12-15

#### Changed

- Refactor error handling, the `Error` type replaces all the error traits.
- `FromBytes` implementation for array does not require to be `Copy + Default`.

#### Added

- Implement `ToBytes` for `String`
- nuts-bytes-derive: The following attributes were added:
  * `map`, `map_from_bytes`, `map_to_bytes`
  * `skip`
  * `default`

#### Removed

- nuts-bytes-derive: Remove the `validate` attribute, replaced by `map`.

### 0.2.1 - 2023-11-17

#### Changed

- Fix some documentation

### 0.2.0 - 2023-11-17

#### Changed

- Removed serde dependency

### 0.1.1 - 2023-10-02

#### Changed

- Update dependencies:
  * `serde` 1.0.152 -> 1.0.188

### 0.1.0 - 2023-06-09

Initial public release.

## nuts-container

### 0.2.2 - 2023-12-05

#### Changed

- Drop Serde dependency, refactoring to support `nuts-bytes` v0.2.2.
- Update dependencies:
  * `nuts-bytes` 0.1.1 -> 0.2.2

### 0.2.1 - 2023-10-13

#### Added

- `MemoryBackend` has a variable block size.

### 0.2.0 - 2023-10-10

#### Changed

- `Backend::aquire()` has now a `buf` argument.
- New internal en-/decryption API: `CipherContext` replaces methods on the
  `Cipher` type.

### 0.1.1 - 2023-10-02

#### Changed

- Update dependencies:
  * `log` 0.4.14 -> 0.4.20
  * `nuts-bytes` 0.1.0 -> 0.1.1
  * `serde` 1.0.152 -> 1.0.188

### 0.1.0 - 2023-08-29

Initial public release.

## nuts-directory

### 0.3.2 - 2023-12-05

#### Changed

- Removed Serde dependency
- Update dependencies:
  * `nuts-container` 0.2.0 -> 0.2.2
  * `nuts-bytes` 0.1.1 -> 0.2.2

### 0.3.1 - 2023-11-10

#### Changed
- Make the write operation atomic

### 0.3.0 - 2023-10-25

#### Changed
- The backend handles a generic `Path` (instead of an owned `PathBuf`).

### 0.2.0 - 2023-10-10

#### Changed

- New `std::fmt::Debug` implementation for `Id`.
- `nuts_container::backend::Backend` changed its interface.
- Update dependencies:
  * `nuts-container` 0.1.1 -> 0.2.0

### 0.1.1 - 2023-10-02

#### Changed
- Update dependencies:
  * `getrandom` 0.2.8 -> 0.2.10
  * `log` 0.4.14 -> 0.4.20
  * `nuts-container` 0.1.0 -> 0.1.1
  * `nuts-bytes` 0.1.0 -> 0.1.1
  * `serde` 1.0.152 -> 1.0.188

#### Fixed

- Backend cannot be created without overwrite option set

### 0.1.0 - 2023-10-02

Initial public release.

## nuts-tool

### 0.3.8 - 2023-12-06

#### Changed

- Update dependencies:
  * `nuts-archive` 0.2.3 -> 0.2.4
  * `nuts-container` 0.2.1 -> 0.2.2
  * `nuts-directory` 0.3.1 -> 0.3.2

### 0.3.7 - 2023-11-10

#### Changed

- Update dependencies:
  * `nuts-directory` 0.3.0 -> 0.3.1

### 0.3.6 - 2023-11-10

#### Added

- Global `--quiet` option to suppress all output
- New command `archive get` to retrieve the content of an entry

#### Changed

- Update dependencies:
  * `nuts-archive` 0.2.2 -> 0.2.3

### 0.3.5 - 2023-11-07

#### Fixed

- Fix build on Ubuntu 22.04

### 0.3.4 - 2023-11-07

#### Added

- Put timestamps & permissions into entries while appending to the archive

#### Changed

- Update dependencies:
  * `nuts-archive` 0.2.ß -> 0.2.2
- `--time-format` option for `archive info`

### 0.3.3 - 2023-11-03

#### Changed

- Update dependencies:
  * `nuts-archive` 0.1.1 -> 0.2.0
- `archive add` command can directly append filesystem entries
- `--long` option for `archive list`

#### Added

- `archive add file|directory|symlink` commands can archive entries manually.

### 0.3.2 - 2023-10-30

#### Added

- `archive create` command can directly append filesystem entries

#### Changed

- Collect versions for various dependent packages for `--version`
- Finetuning of `--verbose` level

### 0.3.1 - 2023-10-27

#### Changed

- Update dependencies:
  * `nuts-archive` 0.1.0 -> 0.1.1

### 0.3.0 - 2023-10-25

#### Added

- Initial integration of nuts-archive

### 0.2.1 - 2023-10-25

#### Changed

- Update dependencies:
  * `nuts-container` 0.1.1 -> 0.2.1
  * `nuts-directory` 0.1.1 -> 0.3.0

### 0.2.0 - 2023-10-10

#### Added

- New sub-commands `aquire` & `release` for the `container` command.

### 0.1.0 - 2023-10-02

Initial public release.

[0.4.0]: https://github.com/drobin/nuts/tree/v0.4.0
[0.4.1]: https://github.com/drobin/nuts/tree/v0.4.1
[0.4.2]: https://github.com/drobin/nuts/tree/v0.4.2
[0.4.3]: https://github.com/drobin/nuts/tree/v0.4.3
[0.5.0]: https://github.com/drobin/nuts/tree/v0.5.0
[0.6.0]: https://github.com/drobin/nuts/tree/v0.6.0
[0.6.1]: https://github.com/drobin/nuts/tree/v0.6.1
[0.6.2]: https://github.com/drobin/nuts/tree/v0.6.2
[0.6.3]: https://github.com/drobin/nuts/tree/v0.6.3
[0.6.4]: https://github.com/drobin/nuts/tree/v0.6.4
[0.6.5]: https://github.com/drobin/nuts/tree/v0.6.5
