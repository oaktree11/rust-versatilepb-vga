// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

//! Conditional re-exporting of Board Support Packages.

#[cfg(any(feature = "bsp_versatilepb", feature = "bsp_rpi4"))]
mod versatilepb;

#[cfg(any(feature = "bsp_versatilepb", feature = "bsp_rpi4"))]
pub use versatilepb::*;
