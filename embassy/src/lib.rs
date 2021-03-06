#![cfg_attr(not(feature = "std"), no_std)]
#![feature(slice_fill)]
#![feature(generic_associated_types)]
#![feature(const_fn)]
#![feature(const_fn_fn_ptr_basics)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

pub mod executor;
pub mod flash;
pub mod io;
pub mod rand;
pub mod time;
pub mod util;
