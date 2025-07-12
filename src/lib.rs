//! # SLUGENCODE
//! 
//! The SLUGENCODE library provides functionality to encode bytes as different types such as hex, base32, base58, base64, and more
//! 
//! ## Features
//! 
//! - Encodings (Hex, Base32, Base58, Base64, Bytes)
//! - Large Unsigned Integer (feature)
//! - Zeroize (feature)
//! - integrity-check (feature) (uses BLAKE2s)
//! 
//! 


// TODO:
// [X] From_Hex
// [X] From_Bs64_URL
// [X] From_Bs64
// [] From_bs32
// [] From_bs32_unpadded


// [] From Base32
// [] From Base58

// [X] To_Hex
// [X] To_Bs64_URL
// [X] To_Bs64
// [X] To_Base58 (not ct)
// [] To_bs32
// [] To_bs32_unpadded

// Add u64, u128, u256 ...

// Hexadecimal
use ct_codecs::{Decoder, Encoder, Hex};

// Base64
use ct_codecs::Base64UrlSafe;
use ct_codecs::Base64;

// Base32
use base32ct::Error as Bs32Error;
use base32ct::{Base32,Base32Unpadded};
use base32ct::Encoding;

// Base58
use bs58;
use bs58::encode::Error as bs58Error;

// Errors
use ct_codecs::Error;

pub trait SlugEncoder {
    /// # \[Constant-Time] To Hexadecimal
    /// 
    /// Uses `ct_codecs` crate to convert bytes to hexadecimal.
    /// 
    /// Accepts as input `AsRef<[u8]>`
    fn to_hex(&self) -> Result<String, Error>;
    /// # \[Constant-Time] To Base64 (URL SAFE) (With Padding)
    /// 
    /// Uses `ct_codecs` crate to convert bytes to base64 url safe string.
    /// 
    /// Accepts as input `AsRef<[u8]>`
    fn to_bs64_url(&self) -> Result<String, Error>;
    /// # \[Constant-Time] To Base64
    /// 
    /// Uses `ct_codecs` crate to convert bytes to base64 string.
    /// 
    /// Accepts as input `AsRef<[u8]>`
    fn to_bs64(&self) -> Result<String, Error>;

    fn to_bs32(&self) -> Result<String, Bs32Error>;
    fn to_bs32_unpadded(&self) -> Result<String, Bs32Error>;
    fn to_base58(&self) -> String;
}

/// # SlugEncode
/// 
/// The trait used to convert the encoding.
pub trait SlugEncode {
    fn as_bytes(&self);
    fn to_bytes(&self);
    
    /// # \[Constant-Time] To Hexadecimal
    /// 
    /// Uses `ct_codecs` crate to convert bytes to hexadecimal.
    /// 
    /// Accepts as input `AsRef<[u8]>`
    fn to_hex<T: AsRef<[u8]>>(&self, bytes: T) -> Result<String, Error> {
        let hex_str = Hex::encode_to_string(bytes.as_ref())?;
        Ok(hex_str)
    }
    fn to_bs32();
    fn to_bs58();
    /// # \[Constant-Time] To Base64 (URL SAFE) (With Padding)
    /// 
    /// Uses `ct_codecs` crate to convert bytes to base64 url safe string.
    /// 
    /// Accepts as input `AsRef<[u8]>`
    fn to_bs64_url<T: AsRef<[u8]>>(&self, bytes: T) -> Result<String, Error> {
        let bs64_url_str = Base64UrlSafe::encode_to_string(bytes.as_ref())?;
        Ok(bs64_url_str)
    }
    /// # \[Constant-Time] To Base64
    /// 
    /// Uses `ct_codecs` crate to convert bytes to base64 string.
    /// 
    /// Accepts as input `AsRef<[u8]>`
    fn to_bs64<T: AsRef<[u8]>>(&self, bytes: T) -> Result<String, Error> {
        let bs64_url_str = Base64::encode_to_string(bytes.as_ref())?;
        Ok(bs64_url_str)
    }

    /// # \[Constant-Time] From Hexadecimal
    /// 
    /// Uses `ct_codecs` crate to convert a hexadecimal string into a vector of bytes.
    /// 
    /// Returns a `ct_codecs` Error if something goes wrong.
    /// 
    /// Accepts as input String or &str
    /// 
    /// ```rust
    /// use slugencode::{SlugEncode,SlugEncoder};
    /// 
    /// fn main() {
    ///     let hex_str: &str = "FFFFFFFF";
    /// }
    /// ```
    fn from_hex<T: AsRef<str>>(&self, hex_str: T) -> Result<Vec<u8>,Error> {
        let bytes = Hex::decode_to_vec(hex_str.as_ref(), None)?;
        Ok(bytes)
    }
    fn from_bs32();
    fn from_bs58();

    /// # \[Constant-Time] From Base64 (URL SAFE) (With Padding)
    /// 
    /// ## Description
    /// 
    /// **Note:** Base64 (URL SAFE) uses `_` and `-` as opposed to `+` and `/`
    /// 
    /// Uses `ct_codecs` crate to convert a base64 (url safe) string into a vector of bytes
    /// 
    /// Returns a `ct_codecs` Error if something goes wrong.
    /// 
    /// Accepts as input `String` or `&str`
    fn from_bs64_url<T: AsRef<str>>(&self, bs64_url_safe_str: T) -> Result<Vec<u8>,Error> {
        let bytes = Base64UrlSafe::decode_to_vec(bs64_url_safe_str.as_ref(), None)?;
        Ok(bytes)
    }
    /// # \[Constant-Time] From Base64
    /// 
    /// ## Description
    /// 
    /// Uses `ct_codecs` crate to convert a base64 string into a vector of bytes
    /// 
    /// Returns a `ct_codecs` Error if something goes wrong.
    /// 
    /// Accepts as input `String` or `&str`
    fn from_bs64<T: AsRef<str>>(&self, bs64: T) -> Result<Vec<u8>,Error> {
        let bytes = Base64::decode_to_vec(bs64.as_ref(), None)?;
        Ok(bytes)
    }
}

/*
pub struct SlugEncoder;

impl SlugEncode for SlugEncoder {
    fn from_hex<T: AsRef<str>>(hex_str: T) -> Result<Vec<u8>,Error> {
        
    }
}
*/

impl SlugEncoder for Vec<u8> {
    fn to_hex(&self) -> Result<String, Error> {
        let hex_str = Hex::encode_to_string(&self)?;
        Ok(hex_str)
    }
    fn to_bs64(&self) -> Result<String, Error> {
        let bs64_url_str = Base64::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs64_url(&self) -> Result<String, Error> {
        let bs64_url_str = Base64UrlSafe::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs32(&self) -> Result<String, Bs32Error> {
        let mut slice: [u8;32] = [0u8;32];
        let bs32 = Base32::encode(&self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_bs32_unpadded(&self) -> Result<String, Bs32Error> {
        let mut slice: [u8;32] = [0u8;32];
        let bs32 = Base32Unpadded::encode(&self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_base58(&self) -> String {
        let s = bs58::encode(&self).into_string();
        return s
    }
}

impl SlugEncoder for &[u8] {
    fn to_hex(&self) -> Result<String, Error> {
        let hex_str = Hex::encode_to_string(&self)?;
        Ok(hex_str)
    }
    fn to_bs64(&self) -> Result<String, Error> {
        let bs64_url_str = Base64::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs64_url(&self) -> Result<String, Error> {
        let bs64_url_str = Base64UrlSafe::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs32(&self) -> Result<String, Bs32Error> {
        let mut slice: [u8;32] = [0u8;32];
        let bs32 = Base32::encode(&self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_bs32_unpadded(&self) -> Result<String, Bs32Error> {
        let mut slice: [u8;32] = [0u8;32];
        let bs32 = Base32Unpadded::encode(&self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_base58(&self) -> String {
        let s = bs58::encode(&self).into_string();
        return s
    }
}

impl SlugEncoder for [u8;28] {
    fn to_hex(&self) -> Result<String, Error> {
        let hex_str = Hex::encode_to_string(&self)?;
        Ok(hex_str)
    }
    fn to_bs64(&self) -> Result<String, Error> {
        let bs64_url_str = Base64::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs64_url(&self) -> Result<String, Error> {
        let bs64_url_str = Base64UrlSafe::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs32(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_bs32_unpadded(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32Unpadded::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_base58(&self) -> String {
        let s = bs58::encode(&self).into_string();
        return s
    }
}

impl SlugEncoder for [u8;32] {
    fn to_hex(&self) -> Result<String, Error> {
        let hex_str = Hex::encode_to_string(&self)?;
        Ok(hex_str)
    }
    fn to_bs64(&self) -> Result<String, Error> {
        let bs64_url_str = Base64::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs64_url(&self) -> Result<String, Error> {
        let bs64_url_str = Base64UrlSafe::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
        fn to_bs32(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_bs32_unpadded(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32Unpadded::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_base58(&self) -> String {
        let s = bs58::encode(&self).into_string();
        return s
    }
}

impl SlugEncoder for [u8;48] {
    fn to_hex(&self) -> Result<String, Error> {
        let hex_str = Hex::encode_to_string(&self)?;
        Ok(hex_str)
    }
    fn to_bs64(&self) -> Result<String, Error> {
        let bs64_url_str = Base64::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs64_url(&self) -> Result<String, Error> {
        let bs64_url_str = Base64UrlSafe::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs32(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_bs32_unpadded(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32Unpadded::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_base58(&self) -> String {
        let s = bs58::encode(&self).into_string();
        return s
    }
}

impl SlugEncoder for [u8;64] {
    fn to_hex(&self) -> Result<String, Error> {
        let hex_str = Hex::encode_to_string(&self)?;
        Ok(hex_str)
    }
    fn to_bs64(&self) -> Result<String, Error> {
        let bs64_url_str = Base64::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs64_url(&self) -> Result<String, Error> {
        let bs64_url_str = Base64UrlSafe::encode_to_string(&self)?;
        Ok(bs64_url_str)
    }
    fn to_bs32(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_bs32_unpadded(&self) -> Result<String, Bs32Error> {
        let mut slice: Vec<u8> = vec![];
        let bs32 = Base32Unpadded::encode(self, &mut slice)?;
        Ok(bs32.to_string())
    }
    fn to_base58(&self) -> String {
        let s = bs58::encode(&self).into_string();
        return s
    }
}


pub struct SlugAPI;

impl SlugAPI {
    pub fn new() {
        // TODO
    }
}




#[test]
fn run() {
    use self::SlugEncoder;
    let bytes: [u8;30] = [33u8;30];
    let byte_slice = bytes.as_slice();
    let output = byte_slice.to_base58();

    println!("Output: {}", output);
}