// SPDX-License-Identifier: Apache-2.0

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::Extended;

#[cfg(target_arch = "x86")]
pub use x86::Basic;

#[cfg(target_arch = "x86_64")]
pub use x86_64::Basic;
