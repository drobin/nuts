# nuts-bytes: Conversion into a binary data format.

The `nuts-bytes` crate implements a tool that converts structured data into a
binary format.

## API documentation

The API documentation is located [here](https://docs.rs/nuts-bytes/).

## Format specification

The binary format is specified in [docs/format.md].

## Deserialization example

```rust
use nuts_bytes::{Error, Reader, TakeBytesError};

// deserialize a primitive (u32)
let mut reader = Reader::new([0x00, 0x00, 0x02, 0x9A].as_slice());
let n: u32 = reader.read().unwrap();

assert_eq!(n, 666);

// Not enough data available
let mut reader = Reader::new([0; 3].as_slice());
let err = reader.read::<u32>().unwrap_err();

assert!(matches!(err, Error::TakeBytes(TakeBytesError::Eof)));
```

## Serialization example

```rust
use nuts_bytes::Writer;

// serialize a primitive (u32)
let mut writer = Writer::new(vec![]);
writer.write(&666u32).unwrap();

assert_eq!(writer.into_target(), [0x00, 0x00, 0x02, 0x9A]);
```
## License

> You can check out the full license
> [here](https://github.com/drobin/nuts/blob/master/LICENSE).

This project is licensed under the terms of the **MIT** license.

[docs/format.md]: https://github.com/drobin/nuts/blob/master/nuts-bytes/docs/format.md
