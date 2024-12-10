use binary_sv2::{
    decodable::FieldMarker,
    encodable::{EncodableField, EncodablePrimitive},
    Error, GetMarker, GetSize, Seq064K, Sv2DataType, B032, B064K,
};

use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2;
#[cfg(feature = "with_serde")]
use binary_sv2::GetSize;
use binary_sv2::{Deserialize, Serialize};
#[cfg(not(feature = "with_serde"))]
use core::convert::TryInto;

/// This is the data type to be sent by a proxy to the pool.
/// We don't want an explicit "Share" type since ASICs cannot create
/// the blinded message part.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[already_sized]
#[repr(C)]
pub struct BlindedMessageShare<'decoder> {
    pub job_id: u64,
    pub nonce: u32,
    pub ntime: u32,
    pub version: u32,
    // begin eHash data
    pub amount: u64,
    pub keyset_id: u64,
    pub parity_bit: bool,
    pub blinded_secret: PubKey<'decoder>,
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub extranonce: B032<'decoder>,
}

impl<'a> BlindedMessageShare<'a> {
    pub fn seq_into_encodable_field(v: Seq064K<'a, BlindedMessageShare<'a>>) -> EncodableField<'a> {
        let inner = v.into_inner();
        let inner_len = inner.len() as u16;
        let mut as_encodable: Vec<EncodableField> = Vec::with_capacity((inner_len + 2) as usize);
        as_encodable.push(EncodableField::Primitive(EncodablePrimitive::OwnedU8(
            inner_len.to_le_bytes()[0],
        )));
        as_encodable.push(EncodableField::Primitive(EncodablePrimitive::OwnedU8(
            inner_len.to_le_bytes()[1],
        )));
        for element in inner {
            as_encodable.push(element.into());
        }
        EncodableField::Struct(as_encodable)
    }
}

impl<'d> GetMarker for BlindedMessageShare<'d> {
    fn get_marker() -> FieldMarker {
        let markers = vec![
            u64::get_marker(),
            u32::get_marker(),
            u32::get_marker(),
            u32::get_marker(),
            u64::get_marker(),
            u64::get_marker(),
            bool::get_marker(),
            PubKey::get_marker(),
            B032::get_marker(),
        ];
        FieldMarker::Struct(markers)
    }
}

impl<'d> GetSize for BlindedMessageShare<'d> {
    fn get_size(&self) -> usize {
        self.extranonce.get_size()
            + self.job_id.get_size()
            + self.nonce.get_size()
            + self.merkle_path.get_size()
            + self.ntime.get_size()
            + self.reference_job_id.get_size()
            + self.share_index.get_size()
            + self.version.get_size()
            + self.amount.get_size()
            + self.keyset_id.get_size()
            + self.parity_bit.get_size()
            + self.blinded_secret.get_size()
    }
}

impl<'d> binary_sv2::SizeHint for BlindedMessageShare<'d> {
    // This is not needed
    fn size_hint(_: &[u8], _: usize) -> Result<usize, binary_sv2::Error> {
        todo!()
    }
    fn size_hint_(&self, _: &[u8], _: usize) -> Result<usize, binary_sv2::Error> {
        Ok(self.get_size())
    }
}

// TODO: test this
impl<'d> TryInto<binary_sv2::decodable::FieldMarker> for BlindedMessageShare<'d> {
    type Error = ();
    fn try_into(self) -> Result<binary_sv2::decodable::FieldMarker, Self::Error> {
        Ok(BlindedMessageShare::get_marker())
    }
}

impl<'d> Sv2DataType<'d> for BlindedMessageShare<'d> {
    fn from_bytes_unchecked(data: &'d mut [u8]) -> Self {
        let job_id = u64::from_bytes_unchecked(&mut data[0..8]);
        let nonce = u32::from_bytes_unchecked(&mut data[8..12]);
        let ntime = u32::from_bytes_unchecked(&mut data[12..16]);
        let version = u32::from_bytes_unchecked(&mut data[16..20]);
        let amount = u64::from_bytes_unchecked(&mut data[20..28]);
        let keyset_id = u64::from_bytes_unchecked(&mut data[28..36]);
        let parity_bit = bool::from_bytes_unchecked(&mut data[36..37]);
        let blinded_secret = PubKey
            ::from_bytes_unchecked(&mut data[37..69]);
        let extranonce = B032::from_bytes_unchecked(&mut data[69..]);
        let extranonce = extranonce.into_static();
        Self {
            job_id,
            nonce,
            ntime,
            version,
            amount,
            keyset_id,
            parity_bit,
            blinded_secret,
            extranonce,
        }
    }

    fn from_vec_(_data: Vec<u8>) -> Result<Self, Error> {
        unreachable!()
    }

    fn from_vec_unchecked(_data: Vec<u8>) -> Self {
        unreachable!()
    }

    fn to_slice_unchecked(&'d self, dst: &mut [u8]) {
        debug_assert!(dst.len() == 60);
        self.job_id.to_slice_unchecked(&mut dst[0..8]);
        self.nonce.to_slice_unchecked(&mut dst[8..12]);
        self.ntime.to_slice_unchecked(&mut dst[12..16]);
        self.version.to_slice_unchecked(&mut dst[16..20]);
        self.amount.to_slice_unchecked(&mut dst[20..28]);
        self.keyset_id.to_slice_unchecked(&mut dst[28..36]);
        self.parity_bit.to_slice_unchecked(&mut dst[36..37]);
        self.blinded_secret.to_slice_unchecked(&mut dst[37..69]);
        self.extranonce.to_slice_unchecked(&mut dst[69..]);
    }
}
