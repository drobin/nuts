# nuts: A secure storage toolset.

## Introduction

The _nuts_ library implements a secure storage library, where data are stored
in a container. The container is divided into encrypted blocks. So a _nuts_
container looks like a block device. All the things (e.g. keys) you need to
open it are stored in the container itself. The library has a rich API, so it
can be easily integrated into you application.

The container does not manage the encrypted data itself. It is transferred to a
backend that is solely responsible for the persistent storage of the blocks. In
this way, the data can easily be stored on different media or in different
formats. The keys remain in the container and do not have to be stored in the
backend.

## [nuts-container]

The [nuts-container] implements the container itself.

## Backend implementations

### [nuts-directory]

The [nuts-directory] crate implements a nuts backend where the blocks of the
container are stored in a file hierarchy. Each block is identified by an id,
which is basically a 16 byte random number.

## Applications

### [nuts-archive]

The nuts-archive is an application based on the nuts container. Inspired by the
`tar` tool you can store files, directories and symlinks in a nuts container.

## Tools

The [nuts-tool] projects implements a commandline utility to maintain nuts
container and its application.

[nuts-archive]: https://github.com/drobin/nuts/tree/master/nuts-archive
[nuts-container]: https://github.com/drobin/nuts/tree/master/nuts-container
[nuts-directory]: https://github.com/drobin/nuts/tree/master/nuts-directory
[nuts-tool]: https://github.com/drobin/nuts/tree/master/nuts-tool
