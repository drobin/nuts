# nuts-tool: A commandline utility for [nuts] container.

## Introduction

The _nuts-tool_ project implements a commandline utility to maintain [nuts]
container. The container has an attached [directory backend].

```
$ # create a nuts container
$ nuts container create sample
Enter a password:
```

```
$ # lists all container
$ nuts container list
sample
```

```
$ # display information about a container
$ NUTS_CONTAINER=sample nuts container info
Enter a password:
cipher:             aes128-gcm
kdf:                pbkdf2:sha1:65536:16
block size (gross): 512
block size (net):   496
```

## License

> You can check out the full license
> [here](https://github.com/drobin/nuts-container/blob/master/LICENSE).

This project is licensed under the terms of the **MIT** license.

[nuts]: https://github.com/drobin/nuts-container
[directory backend]: https://github.com/drobin/nuts-directory
