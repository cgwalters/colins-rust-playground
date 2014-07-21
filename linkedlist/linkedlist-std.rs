// Copyright 2014 Colin Walters <walters@verbum.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate collections;
use collections::{DList, Deque};
    
#[test]
fn tests() {
    let mut l : DList<uint> = DList::new();

    assert_eq!(l.is_empty(), true);
    assert_eq!(l.front(), None);

    l.push_front(42u);

    assert_eq!(l.is_empty(), false);
    assert_eq!(l.front(), Some(&42u));
}
