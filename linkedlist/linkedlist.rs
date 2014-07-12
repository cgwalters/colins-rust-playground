// Copyright 2014 Colin Walters <walters@verbum.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct DoubleLinkedList<'a T> {
    prev: Option<&'a DoubleLinkedList<T>>,
    next: Option<&'a DoubleLinkedList<T>>,
    val: &T,
}

impl<T> DoubleLinkedList<T> {
    fn new<T>(val: T) {
        DoubleLinkedList { prev: None, next: None, val: val };
    }
    
    fn prepend(&mut self, val: T) {
        DoubleLinkedList { prev: None, next: self, val: val };
    }

    fn prev<'a>(&'a self) -> &'a Option<DoubleLinkedList<T>> {
        self.prev;
    }

    fn next<'a>(&'a self) -> &'a Option<DoubleLinkedList<T>> {
        self.next;
    }

    fn get(&self) -> &T {
        self.val;
    }
}
    
#[test]
fn test_empty() {
    let l : DoubleLinkedList<uint> = DoubleLinkedList::new(42u);

    assert_eq!(l.get(), 42u);
    assert_eq!(l.next(), None);
    assert_eq!(l.prev(), None);
}
