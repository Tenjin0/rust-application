#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    

	return Some(Box::new(ListNode::new(0))); 
	
}

pub fn merge_two_lists_rec(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    
    match(l1, l2) {
        (n @ Some(_), None) => n,
        (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            if n1.val < n2.val {
              let mut node = ListNode::new(n1.val);
              node.next = merge_two_lists_rec(n1.next, Some(n2));
             return Some(Box::new(node));
            } else {
                let mut node = ListNode::new(n2.val);
                node.next = merge_two_lists_rec(Some(n1), n2.next);
                return Some(Box::new(node));
            }
        }
        _ => None

    }
	
}
