use merge::{ListNode, merge_two_lists};

fn main() {
    
    let mut node3 = ListNode::new(4);
    let mut node2 = ListNode::new(2);
    let mut node1 = ListNode::new(1);

    node2.next = Option::Some(Box::new(node3));
    node1.next = Option::Some(Box::new(node2));
    
    let node6 = ListNode::new(4);
    let mut node5 = ListNode::new(3);
    let mut node4 = ListNode::new(1);

    node5.next = Option::Some(Box::new(node6));
    node4.next = Option::Some(Box::new(node5));

    merge_two_lists(Some(Box::new(node1)), Some(Box::new(node4)));

}
