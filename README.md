# SlugEncode

<img height="25%" width="25%" src="https://sileneundula.github.io/Static/Images/imgs/slugcrypt_final-01.png">

**Author:** [silene, 0x20CB, DionysianMYST]

**License:** APACHE 2.0 or MIT

## Description

SlugAPI is a module that provides a unified interface for encoding and decoding various formats such as Hex, Base32, Base58, and Base64 in a constant-time manner. It is designed to be efficient and secure, making it suitable for cryptographic applications.

SlugEncode is an open-source, swiss-army knife for encoding in constant-time for base32, base64, and hexadecimal, as well as encoding for base58. It uses traits to allow the conversion between encodings, as well as provides a builder struct that can be used to encode/decode data in a simple manner. Due to them being constant-time, excluding Base58, it may be more suitable for certain applications.

### Developer Notes

The traits can be implemented or the struct can be used to encode different data easily and more efficiently in constant-time with zeroize support. It includes support for:

## Constant-Time

- [X] Hexadecimal

- [X] Base32

- [X] Base32 (Padded)

- [X] Base64

- [X] Base64 (URL SAFE)


## Not-Constant Time

- [X] Base58

- [] Base85

## TODO

- Add Zeroize (Feature)
- Add Large Unsigned Intergers (Feature)
- Try Get Encoding (Feature)
- Integrity-Check (Feature)
- Expansion on Traits

### Usage

#### SlugEncodingUsage: A Simple Way To Encode/Decode Data

In this example, we import the slugencode prelude to easily take care of encoding/decoding.

```rust
//! In this example, we import the slugencode prelude, take a hex string, and then decode using the "SlugEncodingUsage" struct as opposed to using traits like "TRAIT::SlugEncoder" or "TRAIT::SlugDecoder".
//! 
//! It returns a vector of bytes (that are decoded hexadecimal).
//! 
//! This can be applied to any encodings listed.


use slugencode::prelude::*;

fn main() -> Result<Vec<u8>,SlugEncodingErrors> {
    let hex_str: &str = "e061930f8e7e02e1bef959646be6b18bc5991ff07a60771bb0b8f8f5";
    
    // Using the SlugAPI
    let slug_encoding = SlugEncodingUsage::new(SlugEncoding::Hex);
    let output_bytes = slug_encoding.decode(hex_str)?;
    return Ok(output_bytes)
}

```

#### SlugEncoder/SlugDecoder: Traits That Can Be Implemented To Encode/Decode Data

In this example, we use the traits `slugencoder` and `slugdecoder` to encode/decode data. It encodes data from bytes while it decodes from strings.

Once the trait is imported, it can be used right away.

```rust

// Import Traits
use slugencode::SlugEncoder;
use slugencode::SlugDecoder;

fn main() {
    let x: Vec<u8> = [4C, 41, 53, 54];
    
    // All Tos
    let hex_str = x.to_hex();
    let base32_str = x.to_bs32();
    let base32unpadded_str = x.to_bs32_unpadded();

    let base58 = x.to_base58();
    let base64 = x.to_base64();
    let base64_url_safe = x.to_base64_url();
}

fn decode() {
    // Message (224)
    let message_224: &str = "4b046cf780d891526710c8fdcab78e1f35c188fc278221f330bcaec2";

    // Convert string from hex using trait
    let hex_bytes = message_224.from_hex().expect("Decoding failed");

    let btc_address = "1FfmbHfnpaZjKFvyi1okTjJJusN455paPH";

    let bytes = btc_address.from_base58().expect("Decoding failed");
}
```

## Contributions

Contributions are welcome :)

## License

APACHE 2.0 OR MIT