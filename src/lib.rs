#![feature(core_intrinsics, decl_macro)]
#![no_std]
#![warn(clippy::all, clippy::pedantic)]

extern crate alloc;

pub mod collections;
pub mod formatting;
pub mod locale;
pub mod num;

pub(crate) mod utils;
