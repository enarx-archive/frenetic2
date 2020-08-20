// SPDX-License-Identifier: Apache-2.0

#![feature(naked_functions)]
#![feature(asm)]
#![deny(clippy::all)]

use frenetic::*;

#[test]
fn basic() {
    #[inline(never)]
    fn inner(cnt: &mut usize, ctx: &mut Context) {
        if ctx.save() {
            *cnt += 1;
            unsafe { ctx.load() }
        } else {
            *cnt += 1;
        }

        Context::wipe();
    }

    let mut ctx = Context::default();
    let mut cnt = 0usize;

    inner(&mut cnt, &mut ctx);
    assert_eq!(cnt, 2);
}
