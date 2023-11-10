# Changelog

All notable changes to this project will be documented in this file.

## [0.3.7] - 2023-11-10

## Changed

- Update dependencies:
  * `nuts-directory` 0.3.0 -> 0.3.1

## [0.3.6] - 2023-11-10

### Added

- Global `--quiet` option to suppress all output
- New command `archive get` to retrieve the content of an entry

## Changed

- Update dependencies:
  * `nuts-archive` 0.2.2 -> 0.2.3

## [0.3.5] - 2023-11-07

### Fixed

- Fix build on Ubuntu 22.04

## [0.3.4] - 2023-11-07

## Added

- Put timestamps & permissions into entries while appending to the archive

## Changed

- Update dependencies:
  * `nuts-archive` 0.2.ß -> 0.2.2
- `--time-format` option for `archive info`

## [0.3.3] - 2023-11-03

## Changed

- Update dependencies:
  * `nuts-archive` 0.1.1 -> 0.2.0
- `archive add` command can directly append filesystem entries
- `--long` option for `archive list`

## Added

- `archive add file|directory|symlink` commands can archive entries manually.

## [0.3.2] - 2023-10-30

### Added

- `archive create` command can directly append filesystem entries

### Changed

- Collect versions for various dependent packages for `--version`
- Finetuning of `--verbose` level

## [0.3.1] - 2023-10-27

### Changed

- Update dependencies:
  * `nuts-archive` 0.1.0 -> 0.1.1

## [0.3.0] - 2023-10-25

### Added

- Initial integration of nuts-archive

## [0.2.1] - 2023-10-25

### Changed

- Update dependencies:
  * `nuts-container` 0.1.1 -> 0.2.1
  * `nuts-directory` 0.1.1 -> 0.3.0

## [0.2.0] - 2023-10-10

### Added

- New sub-commands `aquire` & `release` for the `container` command.

## [0.1.0] - 2023-10-02

Initial public release.

[0.1.0]: https://github.com/drobin/nuts-tool/tree/v0.1.0
[0.2.0]: https://github.com/drobin/nuts-tool/tree/v0.2.0
[0.2.1]: https://github.com/drobin/nuts-tool/tree/v0.2.1
[0.3.0]: https://github.com/drobin/nuts-tool/tree/v0.3.0
[0.3.1]: https://github.com/drobin/nuts-tool/tree/v0.3.1
[0.3.2]: https://github.com/drobin/nuts-tool/tree/v0.3.2
[0.3.3]: https://github.com/drobin/nuts-tool/tree/v0.3.3
[0.3.4]: https://github.com/drobin/nuts-tool/tree/v0.3.4
[0.3.5]: https://github.com/drobin/nuts-tool/tree/v0.3.5
[0.3.6]: https://github.com/drobin/nuts-tool/tree/v0.3.6
[0.3.7]: https://github.com/drobin/nuts-tool/tree/v0.3.7
