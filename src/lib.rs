//! # SLUGENCODE
//! 
//! The SLUGENCODE library provides functionality to encode bytes as different types such as hex, base32, base58, base64, and more
//! 
//! ## Features
//! 
//! - Encodings (Hex, Base32, Base58, Base64, Bytes)
//! - Try Get Encoding (using RegEx) (feature)
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
use bs58::decode::Error as bs58DecodingError;

// Errors
use ct_codecs::Error;


pub mod errors;

use errors::SlugEncodingError;

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd,Hash)]
pub enum SlugEncodings {
    Hex,
    Base32,
    Base32unpadded,
    Base58,
    Base64,
    Base64urlsafe,
}

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

    fn to_bs32(&self) -> String;
    fn to_bs32_unpadded(&self) -> String;
    fn to_base58(&self) -> String;
}

pub trait SlugDecoder {
    fn from_hex(&self) -> Result<Vec<u8>,Error>;
    fn from_bs64(&self) -> Result<Vec<u8>,Error>;
    fn from_bs64_url(&self) -> Result<Vec<u8>,Error>;
    fn from_bs32(&self) -> Result<Vec<u8>,Bs32Error>;
    fn from_bs32_unpadded(&self) -> Result<Vec<u8>,Bs32Error>;
    fn from_base58(&self) -> Result<Vec<u8>,bs58DecodingError>;
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


//=====IMPL SLUGENCODER=====//

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
    fn to_bs32(&self) -> String {
        let bs32 = Base32::encode_string(&self);
        return bs32
    }
    fn to_bs32_unpadded(&self) -> String {
        let bs32 = Base32Unpadded::encode_string(&self);
        bs32
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
    fn to_bs32(&self) -> String {
        let bs32 = Base32::encode_string(&self);
        return bs32
    }
    fn to_bs32_unpadded(&self) -> String {
        let bs32 = Base32Unpadded::encode_string(&self);
        bs32
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
    fn to_bs32(&self) -> String {
        let bs32 = Base32::encode_string(self);
        return bs32
    }
    fn to_bs32_unpadded(&self) -> String {
        let bs32 = Base32Unpadded::encode_string(self);
        bs32
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
    fn to_bs32(&self) -> String {
        let bs32 = Base32::encode_string(self);
        return bs32
    }
    fn to_bs32_unpadded(&self) -> String {
        let bs32 = Base32Unpadded::encode_string(self);
        bs32
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
    fn to_bs32(&self) -> String {
        let bs32 = Base32::encode_string(self);
        return bs32
    }
    fn to_bs32_unpadded(&self) -> String {
        let bs32 = Base32Unpadded::encode_string(self);
        bs32
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
    fn to_bs32(&self) -> String {
        let bs32 = Base32::encode_string(self);
        return bs32
    }
    fn to_bs32_unpadded(&self) -> String {
        let bs32 = Base32Unpadded::encode_string(self);
        bs32
    }
    fn to_base58(&self) -> String {
        let s = bs58::encode(&self).into_string();
        return s
    }
}


/// # SlugAPI Usage
/// 
/// A struct for easy use in conversion. Contains the slugencodings enum that holds the different encodings
/// 
/// ## Example Code
/// 
/// ```rust
/// use slugencodings::SlugAPI;
/// use slugencodings::SlugEncodings;
/// 
/// fn main() {
///     let encoder = SlugAPI::new(SlugEncodings::Hex);
/// }
/// ```
/// 

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd,Hash)]
pub struct SlugAPI {
    encoding: SlugEncodings,
}

impl SlugAPI {
    /// Creates a new instance using the intended encoding/decoding
    pub fn new(encoding: SlugEncodings) -> Self {
        return Self {
            encoding: encoding,
        }
    }
    /// Gets encoding
    pub fn get_encoding(&self) -> SlugEncodings {
        return self.encoding
    }
    pub fn encode<T: AsRef<[u8]>>(&self, bytes: T) -> Result<String,SlugEncodingError> {
        match self.encoding {
            SlugEncodings::Hex => {
                let encoding = bytes.as_ref().to_hex();
                match encoding {
                    Ok(v) => return Ok(v),
                    Err(_) => return Err(SlugEncodingError::Failed)
                }
            }
            SlugEncodings::Base32 => {
                Ok(bytes.as_ref().to_bs32())
            }
            SlugEncodings::Base32unpadded => {
                Ok(bytes.as_ref().to_bs32_unpadded())
            }
            SlugEncodings::Base58 => {
                Ok(bytes.as_ref().to_base58())
            }
            SlugEncodings::Base64 => {
                let encoding =  bytes.as_ref().to_bs64();

                match encoding {
                    Ok(v) => return Ok(v),
                    Err(_) => return Err(SlugEncodingError::Failed)
                }
            }
            SlugEncodings::Base64urlsafe => {
                let encoding =  bytes.as_ref().to_bs64_url();

                match encoding {
                    Ok(v) => return Ok(v),
                    Err(_) => return Err(SlugEncodingError::Failed)
                }
            }
        }
    }
}

impl SlugDecoder for String {
    fn from_hex(&self) -> Result<Vec<u8>,Error> {
        let output = Hex::decode_to_vec(&self, None)?;
        Ok(output)
    }
    fn from_bs64(&self) -> Result<Vec<u8>,Error> {
        let output = Base64::decode_to_vec(&self, None)?;
        Ok(output)
    }
    fn from_bs64_url(&self) -> Result<Vec<u8>,Error> {
        let output = Base64UrlSafe::decode_to_vec(&self, None)?;
        Ok(output)
    }
    fn from_bs32(&self) -> Result<Vec<u8>,Bs32Error> {
        let output = Base32::decode_vec(&self)?;
        Ok(output)
    }
    fn from_bs32_unpadded(&self) -> Result<Vec<u8>,Bs32Error> {
        let output = Base32Unpadded::decode_vec(&self)?;
        Ok(output)
    }
    fn from_base58(&self) -> Result<Vec<u8>,bs58DecodingError> {
        let output = bs58::decode(&self).into_vec()?;
        Ok(output)
    }
}

impl SlugDecoder for &str {
fn from_hex(&self) -> Result<Vec<u8>,Error> {
        let output = Hex::decode_to_vec(&self, None)?;
        Ok(output)
    }
    fn from_bs64(&self) -> Result<Vec<u8>,Error> {
        let output = Base64::decode_to_vec(&self, None)?;
        Ok(output)
    }
    fn from_bs64_url(&self) -> Result<Vec<u8>,Error> {
        let output = Base64UrlSafe::decode_to_vec(&self, None)?;
        Ok(output)
    }
    fn from_bs32(&self) -> Result<Vec<u8>,Bs32Error> {
        let output = Base32::decode_vec(&self)?;
        Ok(output)
    }
    fn from_bs32_unpadded(&self) -> Result<Vec<u8>,Bs32Error> {
        let output = Base32Unpadded::decode_vec(&self)?;
        Ok(output)
    }
    fn from_base58(&self) -> Result<Vec<u8>,bs58DecodingError> {
        let output = bs58::decode(&self).into_vec()?;
        Ok(output)
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

#[test]
fn slugencoder() {
    use self::SlugEncoder;

    println!("Running SlugEncoder Tests:");
    println!("");

    let message_concat: &str = "4144675a6e958d60";
    let message_224: &str = "4144675a6e958d600a2d6a859f5b16ab321ec93e47580ec42025be0b";
    let message_512: &str = "62928ef1f4effcfcba350e9e033ed8c07a94066654e1a4be79dd72027f3828480a7c517266a04fd747a8702d7087a298484c525bdd1b54997bb5eca1a6219b58";

    let bytes = message_concat.as_bytes();
    let byte_vec: Vec<u8> = bytes.to_vec();

    println!("HEX: {}",bytes.to_hex().unwrap());
    println!("Base32: {}", bytes.to_bs32());
    println!("Base32_unpadded: {}", bytes.to_bs32_unpadded());
    println!("Base58: {}", bytes.to_base58());
    println!("Base64: {}",bytes.to_bs64().unwrap());
    println!("Base64_URL_SAFE: {}",bytes.to_bs64_url().unwrap());

}

#[test]
fn slugapi() {
    let x = SlugAPI::new(SlugEncodings::Base64);
    let output = x.encode(b"Hnma").unwrap();

    println!("Output: {}", output)
}