use roles_logic_sv2::common_messages_sv2::SetupConnection;
#[cfg(not(feature = "with_serde"))]
use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2;
use binary_sv2::{Deserialize, Serialize};
#[cfg(not(feature = "with_serde"))]
use core::convert::TryInto;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct SetupConnectionMint<'decoder> {
    pub base: SetupConnection<'decoder>,
    pub keyset_id: u64,
}

impl<'decoder> SetupConnectionMint<'decoder> {
    // Delegate method to reuse logic from SetupConnection
    pub fn requires_standard_job(&self) -> bool {
        self.base.requires_standard_job()
    }

    pub fn set_requires_standard_job(&mut self) {
        self.base.set_requires_standard_job()
    }

    pub fn check_flags(&self, available_flags: u32, required_flags: u32) -> bool {
        SetupConnection::check_flags(self.base.protocol, available_flags, required_flags)
    }

    pub fn get_version(&self, min_version: u16, max_version: u16) -> Option<u16> {
        self.base.get_version(min_version, max_version)
    }

    pub fn set_async_job_nogotiation(&mut self) {
        self.base.set_async_job_nogotiation()
    }
}