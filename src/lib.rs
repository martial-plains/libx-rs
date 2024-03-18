#![feature(cfg_match, const_trait_impl, core_intrinsics, decl_macro, effects)]
#![no_std]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    missing_debug_implementations
)]

extern crate alloc;

pub mod ciphers;
pub mod collections;
pub mod formatting;
pub mod locale;
pub mod num;

pub(crate) mod utils;
