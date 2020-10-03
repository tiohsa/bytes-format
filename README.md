# Bytes Format

## Serialize

```rust
        #[derive(Serialize)]
        struct Test {
            b8: u8,
            b16: u16,
            b32: u32,
            b64: u64,
            lu16: LittleEndianU16,
            #[serde(with = "serde_bytes")]
            v8: Vec<u8>,
        }

        let test = Test {
            b8: 0x00u8,
            b16: 0x0102u16,
            b32: 0x03040506u32,
            b64: 0x0708090A0B0C0D0Eu64,
            lu16: LittleEndianU16(0xFFEEu16),
            v8: vec![0x0Fu8, 0x10],
        };
        let expected = [
            0x00u8, 0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07, 0x0A, 0x09,
            0x0C, 0x0B, 0x0E, 0x0D, 0xEE, 0xFF, 0x0F, 0x10,
        ]
        .to_vec();
        assert_eq!(to_bytes(&test).unwrap(), expected);
```

## Deserialize

```rust
#[derive(Deserialize, PartialEq, Debug)]
struct Test {
    b8: u8,
    b16: u16,
    b32: u32,
    b64: u64,
    lu16: LittleEndianU16,
    #[serde(with = "serde_bytes")]
    v8: Vec<u8>,
}

let test = [
    0x00u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
    0x0B, 0x0C, 0x0D, 0x0E, 0xEE, 0xFF, 0x0F, 0x10,
]
.to_vec();

let expected = Test {
    b8: 0x00u8,
    b16: 0x0102u16,
    b32: 0x03040506u32,
    b64: 0x0708090A0B0C0D0Eu64,
    lu16: LittleEndianU16(0xFFEE),
    v8: vec![0x0Fu8, 0x10],
};

assert_eq!(from_bytes::<Test>(&test[..]).unwrap(), expected);
```
