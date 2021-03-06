//! Internal utility functions - note, the api for anything in this module
//! is unstable and may change even for patch versions of this library.

/// utilities for lair build.rs files
pub mod build;

pub mod codec;
/// Wrapper around whatever upstream crate we're using for crypto_box.
/// Currently the crypto_box crate, future likely to be libsodium.
pub mod crypto_box;
pub(crate) mod rayon;
pub mod sign_ed25519;
pub mod tls;
pub mod util;
pub mod wire;
pub mod x25519;
