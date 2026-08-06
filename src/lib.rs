#![cfg_attr(docsrs, feature(doc_cfg))]

mod dynamic;
mod error;
mod primitive;
mod read;
mod types;
#[cfg(any(feature = "vli", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "vli")))]
mod variable;
mod write;

/// Helper traits for borrowing and referencing
pub mod helpers;

pub mod internal {
    pub use super::dynamic::Dynamic;
    pub use super::error::{Error, Result};
    pub use super::primitive::Primitive;
}
pub use read::{at::ReadValAt, val::ReadVal};
pub use types::*;
pub use write::{at::WriteValAt, val::WriteVal};
