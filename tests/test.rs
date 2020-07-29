// SPDX-License-Identifier: Apache-2.0

#![feature(naked_functions)]
#![feature(asm)]
#![deny(clippy::all)]

use frenetic::*;

#[test]
fn basic() {
    #[inline(never)]
    fn inner(cnt: &mut usize, ctx: &mut Basic) {
        if ctx.save() {
            *cnt += 1;
            unsafe { ctx.load() }
        } else {
            *cnt += 1;
        }

        Basic::wipe();
    }

    let mut ctx = Basic::default();
    let mut cnt = 0usize;

    inner(&mut cnt, &mut ctx);
    assert_eq!(cnt, 2);
}

#[test]
#[cfg(target_feature = "sse")]
fn extended() {
    #[inline(never)]
    fn inner(ctx: &mut Extended) {
        let xmm0: f32 = 0.7;
        unsafe { asm!("", in("xmm0") xmm0) };

        ctx.save();

        let xmm0: f32;
        unsafe { asm!("", out("xmm0") xmm0) };
        assert_eq!(xmm0, 0.7);

        Extended::wipe();

        let xmm0: f32;
        unsafe { asm!("", out("xmm0") xmm0) };
        assert_eq!(xmm0, 0.0);

        ctx.load();

        let xmm0: f32;
        unsafe { asm!("", out("xmm0") xmm0) };
        assert_eq!(xmm0, 0.7);
    }

    let mut ctx = Extended::default();
    inner(&mut ctx);
}
