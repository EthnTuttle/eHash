use binary_sv2::{
    decodable::FieldMarker, encodable::{EncodableField, EncodablePrimitive}, Error, GetMarker, GetSize, PubKey, Seq064K, Sv2DataType, B032, B064K
};

use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2;
#[cfg(feature = "with_serde")]
use binary_sv2::GetSize;
use binary_sv2::{Deserialize, Serialize};
#[cfg(not(feature = "with_serde"))]
use core::convert::TryInto;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[already_sized]
#[repr(C)]
pub struct Keyset<'decoder> {
    /// ordered list of cashu pubkeys; index is key for key/value
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub pubkeys: Seq064K<'decoder, PubKey<'decoder>>
}