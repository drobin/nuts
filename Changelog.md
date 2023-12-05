# Changelog

All notable changes to this project will be documented in this file.

## [0.2.2] - 2023-12-15

## Changed

- Refactor error handling, the `Error` type replaces all the error traits.
- `FromBytes` implementation for array does not require to be `Copy + Default`.

## Added

- Implement `ToBytes` for `String`
- nuts-bytes-derive: The following attributes were added:
  * `map`, `map_from_bytes`, `map_to_bytes`
  * `skip`
  * `default`

### Removed

- nuts-bytes-derive: Remove the `validate` attribute, replaced by `map`.

## [0.2.1] - 2023-11-17

## Changed

- Fix some documentation

## [0.2.0] - 2023-11-17

## Changed

- Removed serde dependency

## [0.1.1] - 2023-10-02

## Changed

- Update dependencies:
  * `serde` 1.0.152 -> 1.0.188

## [0.1.0] - 2023-06-09

Initial public release.

[0.1.0]: https://github.com/drobin/nuts-bytes/tree/v0.1.0
[0.1.1]: https://github.com/drobin/nuts-bytes/tree/v0.1.1
[0.2.0]: https://github.com/drobin/nuts-bytes/tree/v0.2.0
[0.2.1]: https://github.com/drobin/nuts-bytes/tree/v0.2.1
[0.2.2]: https://github.com/drobin/nuts-bytes/tree/v0.2.2
