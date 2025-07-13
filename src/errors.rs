#[derive(Clone,Copy,Debug,PartialEq,PartialOrd,Hash)]
pub enum SlugEncodingError {
    Failed,
    EncodingError,
    DecodingError,
}