use binary_sv2::{decodable::{DecodableField, FieldMarker}, from_bytes, Deserialize, EncodableField, GetSize};
use roles_logic_sv2::{parsers::IsSv2Message, Error};

use crate::{r#const::{CHANNEL_BIT_EHASH_SHARE, CHANNEL_BIT_EHASH_SIGNATURE, MESSAGE_TYPE_EHASH_SHARE, MESSAGE_TYPE_EHASH_SIGNATURE}, EHashShare, EHashSignature};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum EHashMessages<'a> {
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    EHashShare(EHashShare<'a>),
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    EHashSignature(EHashSignature<'a>),
}

impl<'a> IsSv2Message for EHashMessages<'a> {
    fn message_type(&self) -> u8 {
        match self {
            EHashMessages::EHashShare(_) => MESSAGE_TYPE_EHASH_SHARE,
            EHashMessages::EHashSignature(_) => MESSAGE_TYPE_EHASH_SIGNATURE,
        }
    }
    fn channel_bit(&self) -> bool {
        match self {
            EHashMessages::EHashShare(_) => CHANNEL_BIT_EHASH_SHARE,
            EHashMessages::EHashSignature(_) => CHANNEL_BIT_EHASH_SIGNATURE,
        }
    }
}

#[cfg(not(feature = "with_serde"))]
impl<'decoder> From<EHashMessages<'decoder>> for EncodableField<'decoder> {
    fn from(value: EHashMessages<'decoder>) -> Self {
        match value {
            EHashMessages::EHashShare(ehash_share) =>ehash_share.into(),
            EHashMessages::EHashSignature(ehash_signature) => ehash_signature.into(),
        }
    }
}

impl GetSize for EHashMessages<'_> {
    fn get_size(&self) -> usize {
        match self {
            EHashMessages::EHashShare(ehash_share) => ehash_share.get_size(),
            EHashMessages::EHashSignature(ehash_signature) => ehash_signature.get_size(),
        }
    }
    
}

#[cfg(not(feature = "with_serde"))]
impl<'decoder> Deserialize<'decoder> for EHashMessages<'decoder> {
    fn get_structure(_v: &[u8]) -> std::result::Result<Vec<FieldMarker>, binary_sv2::Error> {
        unimplemented!()
    }
    fn from_decoded_fields(
        _v: Vec<DecodableField<'decoder>>,
    ) -> std::result::Result<Self, binary_sv2::Error> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(clippy::enum_variant_names)]
pub enum EHashMessagesTypes {
    EHashShare = MESSAGE_TYPE_EHASH_SHARE,
    EHashSignature = MESSAGE_TYPE_EHASH_SIGNATURE
}

impl TryFrom<u8> for EHashMessagesTypes {
    type Error = Error;

    fn try_from(value: u8) -> Result<EHashMessagesTypes, Error> {
        match value {
            MESSAGE_TYPE_EHASH_SHARE => Ok(EHashMessagesTypes::EHashShare),
            MESSAGE_TYPE_EHASH_SIGNATURE => Ok(EHashMessagesTypes::EHashSignature),
            _ => Err(Error::UnexpectedMessage(value))
        }
    }
}

impl<'a> TryFrom<(u8, &'a mut [u8])> for EHashMessages<'a> {
    type Error = Error;

    fn try_from(value: (u8, &'a mut [u8])) -> Result<Self, Self::Error> {
        let message_type: EHashMessagesTypes = value.0.try_into()?;
        match message_type {
            EHashMessagesTypes::EHashShare => {
                let message: EHashShare<'a> = from_bytes(value.1)?;
                Ok(EHashMessages::EHashShare(message))
            }
            EHashMessagesTypes::EHashSignature => {
                let message: EHashSignature<'a> = from_bytes(value.1)?;
                Ok(EHashMessages::EHashSignature(message))
            }
        }
    }
}