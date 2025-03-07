#![no_std]
#![feature(cfg_match, const_trait_impl, decl_macro, portable_simd)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    missing_debug_implementations
)]

extern crate alloc;
extern crate core;

pub mod collections;
pub mod num;
