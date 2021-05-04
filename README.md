# nuts
> A secure storage library.

## Table of contents
* [Introduction](#introduction)
* [Features](#features)
* [Build Process](#build-process)
* [License](#license)

## Introduction

The _nuts_ library implements a secure storage library, where data are stored
in a container. The container is divided into encrypted blocks. So a _nuts_
container looks like a block device. All the things (e.g. keys) you need to
open it are stored in the container itself. The library has a rich API, so it
can be easily integrated into you application.

## Features

The following encryption algorithms are supported:

* AES128-GCM
* AES128-CTR
* None (which basically disables encryption)

The actual key used for encryption of the blocks (and further information) is
encrypted with a PBKDF2 derivated wrapping key, which is based on a password
provided by the user.

You have a self-contained container, which means that all information you need
to open the container are stored in the first block. Some basic information
(like used cipher & wrapping key algorithm) are stored unencrypted, but most of
the information are also encrypted in the header.

The project ships a commandline tool `nuts`, which can be used for basic
debugging and to maintain a container.

## Build Process

The _nuts_ library is written in `Rust`, so `cargo` is used to build the
library.

```sh
# Clone this repository
$ git clone git@github.com:drobin/nuts.git

# Go into the repository
$ cd nuts

# Build the library
$ cargo build

# Runs the testsuite
$ cargo test
```

## License

> You can check out the full license
> [here](https://github.com/drobin/nuts/blob/master/LICENSE).

This project is licensed under the terms of the **MIT** license.
