// Copyright 2014 Colin Walters <walters@verbum.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod mylist {
    struct MyListNode {
        prev: &Option<Box<MyListNode>>,
        next: &Option<Box<MyListNode>>,
        val: uint
    }

    impl MyListNode {
        fn new(val: uint) { MyListNode { prev: None, next: None, val: val })
    }

    pub struct MyList {
        items: &Option<Box<MyListNode>>,
    }

    impl MyList {
        fn new<T>(val) {
            MyList { items: None };
        }
        
        fn prepend(&mut self, val) {
            match self.items {
                None => self.items = Some(box MyListNode::new(val));
                Some(ref value) => {
                    let mut first = MyListNode
                }
            }
            
            MyList { prev: None, next: self, val: val };
        }

        fn prev(self) -> Option<MyList> {
            self.prev;
        }

        fn next(self) -> Option<MyList> {
            self.next;
        }

        fn get(&self) -> &uint {
            &self.val;
        }
    }
}

use mylist;
    
#[test]
fn test_empty() {
    let l : MyList = MyList::new();

    assert_eq!(l.is_empty(), true);
    assert_eq!(l.front(), None);

    l.push_front(42u);

    assert_eq!(l.is_empty(), false);
    assert_eq!(l.front(), Some(&42u));
}
