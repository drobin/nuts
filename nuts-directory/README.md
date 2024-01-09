# nuts-directory: Nuts backend implementation

## Introduction

The _nuts-directory_ crate implements a [nuts] backend where the blocks of
the container are stored in a file hierarchy. Each block is identified by
an id, which is basically a 16 byte random number.

When storing a block to disks the path to the file is derived from the id:

1. The id is converted into a hex string.
2. The path then would be:
   `<first two chars>/<next two chars>/<remaining chars>`

The header of the container is stored in the file
`00/00/0000000000000000000000000000`.
Nuts backend implementation where the blocks of the container are stored
in a file hierarchy.

# Create a new backend instance

The [`CreateOptions`] type is used to create a new backend instance, which
is passed to the [`Container::create`] method. You need at least a
directory where the backend put its blocks. See the [`CreateOptions`]
documentation for further options.

# Open an existing backend

The [`OpenOptions`] type is used to open a backend instance, which is
passed to the [`Container::open`] method. You need the directory where the
backend put its blocks.

## License

> You can check out the full license
> [here](https://github.com/drobin/nuts/blob/master/LICENSE).

This project is licensed under the terms of the **MIT** license.

[nuts]: https://crates.io/crates/nuts-container
[`CreateOptions`]: https://docs.rs/nuts-directory/latest/nuts_directory/struct.CreateOptions.html
[`OpenOptions`]: https://docs.rs/nuts-directory/latest/nuts_directory/struct.OpenOptions.html
[`Container::create`]: https://docs.rs/nuts-container/latest/nuts_container/container/struct.Container.html#method.create
[`Container::open`]: https://docs.rs/nuts-container/latest/nuts_container/container/struct.Container.html#method.open
