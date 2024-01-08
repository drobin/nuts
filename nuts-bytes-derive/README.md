# nuts-bytes: Conversion into a binary data format.

The `nuts-bytes` crate implements a tool that converts structured data into a
binary format.

## API documentation

The API documentation is located [here](https://docs.rs/nuts-bytes/).

## Format specification

The binary format is specified in [docs/format.md].

## Deserialization example

```rust
use nuts_bytes::{Reader, ReaderError};

// deserialize a primitive (u32)
let mut reader = Reader::<&[u8]>::new([0x00, 0x00, 0x02, 0x9A].as_slice());
let n: u32 = reader.read().unwrap();

assert_eq!(n, 666);

// Not enough data available
let mut reader = Reader::<&[u8]>::new([0; 3].as_slice());
let err = reader.read::<u32>().unwrap_err();

assert!(matches!(err, ReaderError::Eof));
```

## Serialization example

```rust
use nuts_bytes::Writer;

// serialize a primitive (u32)
let mut writer = Writer::<Vec<u8>>::new(vec![]);
writer.write(&666u32).unwrap();

assert_eq!(writer.into_target(), [0x00, 0x00, 0x02, 0x9A]);
```

[docs/format.md]: https://github.com/drobin/nuts-bytes/blob/master/docs/format.md
