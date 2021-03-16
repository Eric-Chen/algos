mod add_tn {
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l: Option<Box<ListNode>> = None;
        let r = l;

        while true {
            let v1, v2 = 0, 0;
            match l1 {
                Some(node) => {v1 = node.val}
                _ => {}
            }
            match l2 {
                Some(node) => {v2 = node.val}
                _ => {}
            }
            let v = v1 + v2;

            if v > 9 {
                l.val = l.val + (v - 10);
                l.next = Some(Box::new(ListNode{
                    val: 1,
                    None
                }))
            } else {
                l.val = l.val + v
            }

            if let None = l1 {
                //do nothing
            } else {
                l1 = l1.next
            }

            if let None = l2 {
                //do nothing
            } else {
                l2 = l2.next
            }

            if !l1.is_none() or !l2.is_none() {
                if let None = l.next {
                    l.next = Some(Box::new(ListNode{
                        val: 0,
                        None
                    }))
                } 
            } else {
                break
            }

        }

        r
    }    
}