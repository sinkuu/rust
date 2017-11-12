// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn test(x: u32) -> u32 {
    let y = x;
    y
}

fn test_chain(x: u32) -> u32 {
    let y = x;
    let z = y;
    let w = z;
    w + z
}

fn main() {
    // Make sure the function actually gets instantiated.
    test(0);
}

// END RUST SOURCE
// START rustc.test.CopyPropagation.before.mir
//  bb0: {
//      ...
//      _3 = _1;
//      ...
//      _2 = _3;
//      ...
//      _4 = _2;
//      _0 = _4;
//      ...
//      return;
//  }
// END rustc.test.CopyPropagation.before.mir
// START rustc.test.CopyPropagation.after.mir
//  bb0: {
//      ...
//      _0 = _1;
//      ...
//      return;
//  }
// END rustc.test.CopyPropagation.after.mir
// START rustc.test_chain.CopyPropagation.before.mir
// bb0: {
//     ...
//     _3 = _1;
//     _2 = _3;
//     ...
//     _5 = _2;
//     _4 = _5;
//     ...
//     _7 = _4;
//     _6 = _7;
//     ...
//     _8 = _6;
//     ...
//     _9 = _4;
//     _0 = Add(_8, _9);
//     ...
//     return;
// }
// END rustc.test_chain.CopyPropagation.before.mir
// START rustc.test_chain.CopyPropagation.after.mir
// bb0: {
//     ...
//     _0 = Add(_1, _1);
//     ...
//     return;
// }
// END rustc.test_chain.CopyPropagation.after.mir
