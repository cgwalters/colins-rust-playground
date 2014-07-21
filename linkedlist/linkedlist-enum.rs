// Copyright 2014 Colin Walters <walters@verbum.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod mylist {
    pub enum MyList {
        Nil,
        Node(uint, Box<MyList>)
    }

    pub fn mylist_is_empty(l: &MyList) -> bool {
        match *l {
            Nil => { true }
            Node(_, _) => { false }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::mylist::{MyList,Nil,Node};
    use super::mylist::mylist_is_empty;

    #[test]
    fn test_empty() {
        let l : MyList = Nil;

        assert_eq!(mylist_is_empty(&l), true);
        match l {
            Nil => {}
            Node(_, _) => { fail!("list should be empty") }
        }
    }

    #[test]
    fn test_cons() {
        let l = Node(42u, box Nil);

        match l {
            Nil => { fail!("list should not be empty") }
            Node(ref val, ref next) => {
                assert_eq!(val, &42u);
                assert_eq!(mylist_is_empty(*next), true);
            }
        }
    }
}
