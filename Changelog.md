# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2023-11-03

## Changed

- Move `container::BufContainer` to `pager::Pager`
- Correct handling of `Error::InvalidBlockSize` error
- Put id into `Error::InvalidType` error

## [0.2.0] - 2023-11-03

## Added

- The `Mode` type encodes the type and access rights and of an entry.

## Changed

- Refactoring of `Entry` & `EntryMut` which now includes the `Mode`.

## [0.1.1] - 2023-10-27

## Changed

- Refactoring of the header of the archive. The header block is guarded by a
  magic value.

## [0.1.0] - 2023-10-25

Initial public release.

[0.1.0]: https://github.com/drobin/nuts-archive/tree/v0.1.0
[0.1.1]: https://github.com/drobin/nuts-archive/tree/v0.1.1
[0.2.0]: https://github.com/drobin/nuts-archive/tree/v0.2.0
[0.2.1]: https://github.com/drobin/nuts-archive/tree/v0.2.1
