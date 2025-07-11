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


// [] From Base32
// [] From Base58

// [X] To_Hex
// [X] To_Bs64_URL
// [X] To_Bs64

// Hexadecimal
use ct_codecs::{Decoder, Encoder, Hex};

// Base64
use ct_codecs::Base64UrlSafe;
use ct_codecs::Base64;

// Errors
use ct_codecs::Error;

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

impl SlugEncode for Vec<u8> {
    fn to_bs64<T: AsRef<[u8]>>(&self, bytes: T) -> Result<String, Error> {
        
    }
}