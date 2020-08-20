// SPDX-License-Identifier: Apache-2.0

#[derive(Default)]
#[repr(C, align(16))]
#[cfg(target_arch = "x86")]
pub struct Context([usize; 6]);

#[cfg(target_arch = "x86")]
impl Context {
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
