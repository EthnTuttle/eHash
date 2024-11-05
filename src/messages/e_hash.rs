#[cfg(not(feature = "with_serde"))]
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2;
#[cfg(feature = "with_serde")]
use binary_sv2::GetSize;
use binary_sv2::{Deserialize, PubKey, Serialize};
#[cfg(not(feature = "with_serde"))]
use core::convert::TryInto;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EHashShare<'decoder> {
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub blinded_message: PubKey<'decoder>,
}

#[cfg(feature = "with_serde")]
impl<'d> GetSize for EHashShare<'d> {
    fn get_size(&self) -> usize {
        self.blinded_message.get_size()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EHashSignature<'decoder> {
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub blind_signature: PubKey<'decoder>,
}

#[cfg(feature = "with_serde")]
impl<'d> GetSize for EHashSignature<'d> {
    fn get_size(&self) -> usize {
        self.blind_signature.get_size()
    }
}