#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

mod lib {
    #[cfg(feature = "std")]
    extern crate core;

    pub use core::fmt;
    pub use core::mem;

    #[cfg(not(feature = "std"))]
    #[macro_use]
    extern crate alloc;

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

pub mod vc_2008;

/// Splits the given str into arguments following VC++ 2008 rules
pub use vc_2008::parse as split;
