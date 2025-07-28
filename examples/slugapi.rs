//! SlugAPI is a module that provides a unified interface for encoding and decoding various formats such as Hex, Base32, Base58, and Base64 in a constant-time manner. It is designed to be efficient and secure, making it suitable for cryptographic applications.

// Struct
use slugencode::prelude::*;
// Traits
use slugencode::SlugEncoder;
use slugencode::SlugDecoder;

fn main() {
    // To: My Mother
    // From: Silene
    let data = "Hi Mom, im on the internet!";
    
    // Encode the data in Hex format (constant-time)
    let slugencoding = SlugEncodingUsage::new(SlugEncodings::Hex);
    let encoded = slugencoding.encode(data.as_bytes());

    // Decode the data back to bytes
    let decoded = slugencoding.decode(&encoded.unwrap()).expect("Decoding failed");

    // Quote of the Day (AI Generated)
    let quote_of_the_day:&'static str  = "The only limit to our realization of tomorrow is our doubts of today.";

    // Into Base32 Using Traits
    let base32 = quote_of_the_day.as_bytes().to_bs32();

    // Base32 RFC4648

    assert_eq!(quote_of_the_day.as_bytes().to_bs32(), "krugkidpnzwhsidmnfwws5baorxsa33voiqhezlbnruxuylunfxw4idpmyqhi33nn5zhe33xebuxgidpovzcazdpovrhi4zan5tca5dpmrqxslq=");

    println!("Base32: {}", base32);


}

fn decode() {
    let message_224: &str = "4b046cf780d891526710c8fdcab78e1f35c188fc278221f330bcaec2";

    message_224.from_hex().expect("Decoding failed");


    
    // Decode the message using Hex encoding
    let slugencoding = SlugEncodingUsage::new(SlugEncodings::Hex);
    let decoded = slugencoding.decode(message_224).expect("Decoding failed");

    println!("Decoded: {:?}", decoded);
}
