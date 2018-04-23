// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-pass

#![crate_type = "lib"]

struct Foo;

pub struct Path<T: Bar> {
    inner: T::Slice,
}

pub trait Bar: Sized {
    type Slice: ?Sized;

    fn open(_: &Path<Self>);
}

impl Bar for Foo {
    type Slice = [u8];

    fn open(_: &Path<Self>) {
    }
}

impl Bar for u8 {
    type Slice = ([u8],);

    fn open(_: &Path<Self>) {
    }
}
