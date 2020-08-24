// SPDX-License-Identifier: Apache-2.0

#[derive(Default)]
#[repr(C, align(16))]
pub struct Context([usize; 8]);

impl Context {
    #[naked]
    #[inline(never)]
    pub extern "C" fn wipe() {
        unsafe {
            asm!(
                "xor     rax, rax",
                "xor     rcx, rcx",
                "xor     rdx, rdx",
                "xor     rdi, rdi",
                "xor     rsi, rsi",
                "xor     r8,  r8",
                "xor     r9,  r9",
                "xor     r10, r10",
                "xor     r11, r11",
                "add     rax, rax",
                "cld",
                "ret",
                options(noreturn)
            )
        }
    }

    #[naked]
    #[must_use]
    #[inline(never)]
    pub extern "C" fn save(&mut self) -> bool {
        unsafe {
            asm!(
                "pop    rsi",
                "mov    [rdi + 0x00], r15",
                "mov    [rdi + 0x08], r14",
                "mov    [rdi + 0x10], r13",
                "mov    [rdi + 0x18], r12",
                "mov    [rdi + 0x20], rbx",
                "mov    [rdi + 0x28], rbp",
                "mov    [rdi + 0x30], rsp",
                "mov    [rdi + 0x38], rsi",
                "mov    rax, 1",
                "jmp    rsi",
                options(noreturn)
            )
        }
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn load(&self) -> ! {
        asm!(
            "mov     rax, 0",
            "mov     r15, qword ptr [rdi + 0x00]",
            "mov     r14, qword ptr [rdi + 0x08]",
            "mov     r13, qword ptr [rdi + 0x10]",
            "mov     r12, qword ptr [rdi + 0x18]",
            "mov     rbx, qword ptr [rdi + 0x20]",
            "mov     rbp, qword ptr [rdi + 0x28]",
            "mov     rsp, qword ptr [rdi + 0x30]",
            "jmp          qword ptr [rdi + 0x38]",
            options(noreturn)
        )
    }
}
