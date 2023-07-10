#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#[cfg(not(feature = "std"))]
extern crate alloc;

mod lib {
    #[cfg(feature = "std")]
    extern crate core;

    pub use core::fmt;
    pub use core::mem;

    #[cfg(not(feature = "std"))]
    pub use alloc::string::String;
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(not(feature = "std"))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;

    #[cfg(not(feature = "std"))]
    pub use alloc::borrow::Cow;
    #[cfg(feature = "std")]
    pub use std::borrow::Cow;
}

pub mod cmd_exe;
pub mod powershell;
pub mod vc_2008;

// Test our README examples as part of doctest
#[cfg(doctest)]
doc_comment::doctest!("../README.md");
