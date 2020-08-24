// SPDX-License-Identifier: Apache-2.0

#[derive(Default)]
#[repr(C, align(16))]
pub struct Context([usize; 10]);

impl Context {
    #[naked]
    #[inline(never)]
    pub extern "C" fn wipe() {
        unsafe {
            asm!(
                "xor     rax, rax",
                "xor     rcx, rcx",
                "xor     rdx, rdx",
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
                "pop    rcx",
                "mov    [rdx + 0x00], r15",
                "mov    [rdx + 0x08], r14",
                "mov    [rdx + 0x10], r13",
                "mov    [rdx + 0x18], r12",
                "mov    [rdx + 0x20], rdi",
                "mov    [rdx + 0x28], rsi",
                "mov    [rdx + 0x30], rbx",
                "mov    [rdx + 0x38], rbp",
                "mov    [rdx + 0x40], rsp",
                "mov    [rdx + 0x48], rcx",
                "mov    rax, 1",
                "jmp    rcx",
                options(noreturn)
            )
        }
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn load(&self) -> ! {
        asm!(
            "mov     rax, 0",
            "mov     r15, [rdx + 0x00]",
            "mov     r14, [rdx + 0x08]",
            "mov     r13, [rdx + 0x10]",
            "mov     r12, [rdx + 0x18]",
            "mov     rdi, [rdx + 0x20]",
            "mov     rsi, [rdx + 0x28]",
            "mov     rbx, [rdx + 0x30]",
            "mov     rbp, [rdx + 0x38]",
            "mov     rsp, [rdx + 0x40]",
            "jmp          [rdx + 0x48]",
            options(noreturn)
        )
    }
}
