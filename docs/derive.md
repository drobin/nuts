If the `derive` feature is enabled with `features = ["derive"]`, the crate
provides two derive macros [`FromBytes`](../derive.FromBytes.html) and
[`ToBytes`](../derive.ToBytes.html), which implements the
[`FromBytes`](../trait.FromBytes.html) resp. [`ToBytes`](../trait.ToBytes.html)
trait struct and enum types.

## (De-)serialize a struct

```rust
use nuts_bytes::{FromBytes, Reader, ToBytes, Writer};

#[derive(Debug, FromBytes, PartialEq, ToBytes)]
struct Sample {
    f1: u16,
    f2: u32,
}

// Deserialize the Sample struct

let mut reader = Reader::<&[u8]>::new([0x00, 0x01, 0x00, 0x00, 0x00, 0x02].as_slice());

let sample: Sample = reader.read().unwrap();
assert_eq!(sample, Sample { f1: 1, f2: 2 });

// Serialize the Sample struct

let mut writer = Writer::<Vec<u8>>::new(vec![]);
writer.write(&Sample { f1: 1, f2: 2 }).unwrap();

assert_eq!(writer.into_target(), [0x00, 0x01, 0x00, 0x00, 0x00, 0x02]);
```

## (De-)serialize an enum

```rust
use nuts_bytes::{FromBytes, Reader, ToBytes, Writer};

#[derive(Debug, FromBytes, PartialEq, ToBytes)]
enum Sample {
    V1,
    V2(u16, u32),
}

const V1: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const V2: [u8; 14] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
];

// Deserialize the Sample::V1 variant

let mut reader = Reader::<&[u8]>::new(V1.as_slice());

let sample: Sample = reader.read().unwrap();
assert_eq!(sample, Sample::V1);

// Deserialize the Sample::V2 variant

let mut reader = Reader::<&[u8]>::new(V2.as_slice());

let sample: Sample = reader.read().unwrap();
assert_eq!(sample, Sample::V2(1, 2));

// Serialize the Sample::V1 variant

let mut writer = Writer::<Vec<u8>>::new(vec![]);
writer.write(&Sample::V1).unwrap();

assert_eq!(writer.into_target(), V1);

// Serialize the Sample::V2 variant

let mut writer = Writer::<Vec<u8>>::new(vec![]);
writer.write(&Sample::V2(1, 2)).unwrap();

assert_eq!(writer.into_target(), V2);
```
