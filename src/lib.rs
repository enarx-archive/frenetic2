// SPDX-License-Identifier: Apache-2.0

#![feature(naked_functions)]
#![feature(asm)]
#![no_std]
#![deny(clippy::all)]
#![deny(missing_docs)]

//! Welcome to frenetic!
//!
//! Frenetic is a crate that provides low-level context switch building blocks.
//! It can be used to build higher level primatives like coroutines.

mod arch;

/// A CPU context
///
/// This structure holds the non-volatile state in the platform's calling convention.
#[derive(Default)]
#[repr(transparent)]
pub struct Context(arch::Context);

impl Context {
    /// Clear all basic volatile registers and CPU flags
    #[inline(always)]
    pub fn wipe() {
        arch::Context::wipe()
    }

    /// Saves the non-volatile registers
    ///
    /// NOTE: This function can return more than once.
    ///
    /// This function returns `true` when returning locally from a `save()`
    /// operation and returns `false` when returning remotely from a `load()`
    /// operation.
    #[must_use]
    #[inline(always)]
    pub fn save(&mut self) -> bool {
        self.0.save()
    }

    /// Load a context
    ///
    /// This function stops executing at the current location and resumes
    /// execution wherever `Self::save()` was called (with a `false` return
    /// value).
    ///
    /// # Safety
    ///
    /// This is very unsafe. You MUST have called `Self::save()` before calling
    /// this function. Further, all the pointers in the non-volatile registers
    /// must still be valid.
    #[inline(always)]
    pub unsafe fn load(&self) -> ! {
        self.0.load()
    }
}
