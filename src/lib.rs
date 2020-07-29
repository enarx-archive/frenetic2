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

/// A basic CPU context
///
/// This structure holds the non-volatile registers in the platform's calling convention.
#[derive(Default)]
#[repr(transparent)]
pub struct Basic(arch::Basic);

impl Basic {
    /// Clear all basic volatile registers and CPU flags
    #[inline(always)]
    pub fn wipe() {
        arch::Basic::wipe()
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

/// An extended CPU context
///
/// This structure hold all extended, volatile CPU state.
#[derive(Default)]
#[repr(transparent)]
pub struct Extended(arch::Extended);

impl Extended {
    /// Resets the extended CPU state to its default state
    #[inline(always)]
    pub fn wipe() {
        arch::Extended::wipe()
    }

    /// Saves the current extended CPU state
    #[inline(always)]
    pub fn save(&mut self) {
        self.0.save()
    }

    /// Loads the specified extended CPU state
    #[inline(always)]
    pub fn load(&self) {
        self.0.load()
    }
}
