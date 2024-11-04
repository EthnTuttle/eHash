#![cfg_attr(feature = "no_std", no_std)]

mod r#const;
mod e_hash;
pub mod parser;

pub use e_hash::{EHashShare, EHashSignature};