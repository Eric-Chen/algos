mod add_tn {
    #[derive(Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }
    
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut r: Option<Box<ListNode>> = Some(Box::new(ListNode{
            val: 0,
            next: None
        }));
        let mut ll1 = l1;
        let mut ll2 = l2;
    
        let mut l = &mut r;
        
        loop {
            match l {
                Some(ref mut node) => {
                    let mut v1: i32 = 0;
                    let mut v2: i32 = 0;
                    if let Some(ref lv1) = ll1 {
                        v1 = lv1.val;
                    }
                    if let Some(ref lv2) = ll2 {
                        v2 = lv2.val;
                    }
                    let v = v1 + v2;
                    if v > 9 {
                        node.val = node.val + (v - 10);
                        node.next = Some(Box::new(ListNode{
                            val: 1,
                            next: None
                        }));
                    } else {
                        node.val = node.val + v
                    }
                    
                    ll1 = if let Some(l1v) = ll1 {
                        l1v.next
                    } else {
                        None
                    };
    
                    ll2 = if let Some(l2v) = ll2 {
                        l2v.next
                    } else {
                        None
                    };
    
                    if !ll1.is_none() || !ll2.is_none() {
                        if !node.next.is_none() {
                            node.next = Some(Box::new(ListNode{
                                val: 1,
                                next: None
                            }));
                        }
    
                        l = &mut node.next;
                    } else {
                        break;
                    }
                    
    
                }
                _ => {
                    //do nothing
                    break;
                }
            }
        }
        r
    }
}

use crate::add_tn::*;

fn main() {
    let l1 = Some(Box::new(ListNode{
        val: 1i32,
        next: Some(Box::new(ListNode{
            val: 2i32,
            next: Some(Box::new(ListNode{
                val: 3i32,
                next: Some(Box::new(ListNode{
                    val: 4i32,
                    next: None
                }))
            }))
        }))
    }));

    let l2 = Some(Box::new(ListNode{
        val: 1i32,
        next: Some(Box::new(ListNode{
            val: 2i32,
            next: Some(Box::new(ListNode{
                val: 3i32,
                next: Some(Box::new(ListNode{
                    val: 4i32,
                    next: None
                }))
            }))
        }))
    }));
    
    let mut a = add_two_numbers(l1, l2);
    loop {
        println!("{}", )
        match a {
            Some(node) => {
                println!("{:?}", node);
                a = node.next;
            }
            _ => {
                break;
            }
        }
    }
}

