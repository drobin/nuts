# nuts-bytes: A binary data format for [Serde].

The `nuts-bytes` crate implements a [Serde] data format that converts into
a binary format.

## API documentation

The API documentation is located [here](https://docs.rs/nuts-bytes/).

## Format specification

The binary format is specified in [docs/format.md].

## Deserialization example

```rust
use nuts_bytes::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct SampleStruct {
    f1: u8,
    f2: u16,
};

#[derive(Debug, Deserialize, PartialEq)]
enum SampleEnum {
    V1(u32)
}

// deserialize a primitive (u32)
let mut reader = Reader::new([0x00, 0x00, 0x02, 0x9A].as_slice());

let n: u32 = reader.deserialize().unwrap();
assert_eq!(n, 666);

// deserialize a struct
let mut reader = Reader::new([0x07, 0x02, 0x9A, 0x00].as_slice());

let sample: SampleStruct = reader.deserialize().unwrap();
assert_eq!(sample, SampleStruct{ f1: 7, f2: 666 });
assert_eq!(*reader.as_ref(), [0x00]); // Still one byte left

// deserialize an enum
let mut reader = Reader::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x9A].as_slice());

let sample: SampleEnum = reader.deserialize().unwrap();
assert_eq!(sample, SampleEnum::V1(666));

// Not enough data available
let mut reader = Reader::new([0; 3].as_slice());
let err = reader.deserialize::<u32>().unwrap_err();

assert_eq!(format!("{}", err), "No more bytes are available for reading.");
```

## Serialization example

```rust
use nuts_bytes::Writer;
use serde::Serialize;

#[derive(Serialize)]
struct SampleStruct {
    f1: u8,
    f2: u16,
};

#[derive(Serialize)]
enum SampleEnum {
    V1(u32)
}

// serialize a primitive (u32)
let mut writer = Writer::new(vec![]);
let n = writer.serialize(&666u32).unwrap();

assert_eq!(n, 4); // 4 bytes written
assert_eq!(writer.into_target(), [0x00, 0x00, 0x02, 0x9A]);

// serialize a struct
let sample = SampleStruct{ f1: 7, f2: 666 };
let mut writer = Writer::new(vec![]);
let n = writer.serialize(&sample).unwrap();

assert_eq!(n, 3); // 3 bytes written
assert_eq!(writer.into_target(), [0x07, 0x02, 0x9A]);

// serialize an enum
let sample = SampleEnum::V1(666);
let mut writer = Writer::new(vec![]);
let n = writer.serialize(&sample).unwrap();

assert_eq!(n, 8); // 8 bytes written
assert_eq!(writer.into_target(), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x9A]);
```

[Serde]: https://www.serde.rs
[docs/format.md]: https://github.com/drobin/nuts-bytes/blob/master/docs/format.md
