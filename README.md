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
formats. The keys remain in the container and are not stored in the backend.

The _nuts-archive_ is an application based on the nuts container. Inspired by
the `tar` tool you can store files, directories and symlinks in a _nuts_
container.

The `nuts` commandline utility is used to maintain _nuts_ container and its
application.

## Getting started

### Installation

The easiest way to try out the `nuts` tool is in a Docker container:

```
docker pull dorobin/nuts
```

Then open a shell in a container:

```
docker run -it dorobin/nuts bash
```

```
root@92cdafaf933b:/# # you are inside the container
root@92cdafaf933b:/# # the nuts tool is installed as /usr/local/bin/nuts
root@92cdafaf933b:/# ls /usr/local/bin/nuts
/usr/local/bin/nuts
```

Alternatively, the `nuts` tool and its backend can be installed using
`cargo install`:

Install the _nuts_ tool:

```
cargo install nuts-tool
```

Install the _nuts-directory_ backend as a plugin:

```
cargo install nuts-directory --features=plugin
```

### Configure the plugin

The plugin must be configured for the nuts tool:

```
nuts plugin add directory --path nuts-directory
```

* `directory` is the name of the plugin. You can use this identifier to
  identify the plugin.
* `nuts-directory` is the path to the plugin executable.

**Note**: If you are using the Docker image, this step is not necessary. The
plugin is already configured in the image.

### Create a container

The following command creates a container with the name `sample`, which uses
the `directory` plugin:

```
nuts container create sample --plugin=directory
```

Retrieve some basic information from the container:

```
NUTS_CONTAINER=sample nuts container info
```

The `nuts` tool evaluates the `NUTS_CONTAINER` environment variable to
determine which container to use. Alternatively, the `--container` commandline
option can be used.

### Create an archive on top of the container

```
NUTS_CONTAINER=sample nuts archive create
```

Put a local file into the archive:

```
echo "hello world" > f1.txt
NUTS_CONTAINER=sample nuts archive add f1.txt
```


List the content of the archive:

```
NUTS_CONTAINER=sample nuts archive list
```

Retrieve a file from the archive:

```
NUTS_CONTAINER=sample nuts archive get f1.txt
```
