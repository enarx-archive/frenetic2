// SPDX-License-Identifier: Apache-2.0

use core::mem::size_of;

#[derive(Default)]
#[repr(C, align(16))]
#[cfg(target_arch = "x86")]
pub struct Basic([usize; 6]);

#[cfg(target_arch = "x86")]
impl Basic {
    #[inline(never)]
    pub extern "C" fn wipe() {
        unsafe {
            asm!(
                "xor     eax, eax",
                "xor     ecx, ecx",
                "xor     edx, edx",
                "add     eax, eax",
                "cld",
                out("eax") _,
                out("ecx") _,
                out("edx") _,
            )
        }
    }

    #[naked]
    #[must_use]
    #[inline(never)]
    pub extern "fastcall" fn save(&mut self) -> bool {
        unsafe {
            asm!(
                "pop    edx",
                "mov    [ecx + 0x00], edi",
                "mov    [ecx + 0x04], esi",
                "mov    [ecx + 0x08], ebx",
                "mov    [ecx + 0x0a], ebp",
                "mov    [ecx + 0x10], esp",
                "mov    [ecx + 0x14], edx",
                "mov    eax, 1",
                "jmp    edx",
                options(noreturn)
            )
        }
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "fastcall" fn load(&self) -> ! {
        asm!(
            "mov     eax, 0",
            "mov     edi, [ecx + 0x00]",
            "mov     esi, [ecx + 0x04]",
            "mov     ebx, [ecx + 0x08]",
            "mov     ebp, [ecx + 0x0a]",
            "mov     esp, [ecx + 0x10]",
            "jmp          [ecx + 0x14]",
            options(noreturn)
        )
    }
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
struct Mm([u8; 10], [u8; 6]);

#[derive(Copy, Clone)]
#[repr(C, align(16))]
struct Xmm([u8; 16]);

#[derive(Copy, Clone)]
#[repr(C, align(64))]
struct Legacy {
    fcw: u16,
    fsw: u16,
    ftw: u8,
    reserved: u8,
    fop: u16,
    fip: u64,
    fdp: u64,
    mxcsr: u32,
    mxcsr_mask: u32,
    mm: [Mm; 8],
    xmm: [Xmm; 16],
    unused: [u64; 12],
}

#[derive(Copy, Clone)]
#[repr(C, align(64))]
struct Header {
    xstate_bv: u64,
    xcomp_bv: u64,
    reserved: [u64; 6],
}

#[derive(Copy, Clone)]
#[repr(C, align(4096))]
pub struct XSave {
    legacy: Legacy,
    header: Header,
    extend: [u8; XSave::fill()],
}

impl XSave {
    const DEFAULT: XSave = XSave {
        legacy: Legacy {
            fcw: 0x037F,
            fsw: 0,
            ftw: 0,
            reserved: 0,
            fop: 0,
            fip: 0,
            fdp: 0,
            mxcsr: 0x1F80,
            mxcsr_mask: 0xFFFF,
            mm: [Mm([0; 10], [0; 6]); 8],
            xmm: [Xmm([0; 16]); 16],
            unused: [0; 12],
        },

        header: Header {
            xstate_bv: 0,
            xcomp_bv: 0,
            reserved: [0; 6],
        },

        extend: [0; XSave::fill()],
    };

    const fn fill() -> usize {
        4096 - size_of::<Legacy>() - size_of::<Header>()
    }
}

#[repr(transparent)]
pub struct Extended(XSave);

impl Default for Extended {
    fn default() -> Self {
        Self(XSave::DEFAULT)
    }
}

impl Extended {
    #[inline(always)]
    pub fn wipe() {
        Self(XSave::DEFAULT).load()
    }

    #[inline(never)]
    pub extern "C" fn save(&mut self) {
        unsafe {
            asm!(
                "mov     eax, ~0",
                "mov     edx, ~0",
                "xsave   [{}]",

                in(reg) &mut self.0,
                out("eax") _,
                out("edx") _,
            )
        }
    }

    #[inline(never)]
    pub extern "C" fn load(&self) {
        unsafe {
            asm!(
                "mov     eax, ~0",
                "mov     edx, ~0",
                "xrstor  [{}]",

                in(reg) &self.0,
                out("eax") _,
                out("edx") _,
            )
        }
    }
}

#[cfg(test)]
#[test]
fn size() {
    assert_eq!(size_of::<Header>(), 64);
    assert_eq!(size_of::<Legacy>(), 512);
    assert_eq!(size_of::<XSave>(), 4096);
}
