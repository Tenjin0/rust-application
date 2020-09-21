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
    let mut current_l1 = l1;
    let mut current_l2 = l2;

    if current_l1.is_none() && current_l2.is_none() {
        return None;
    } else if current_l1.is_none() && current_l2.is_some() {
        return current_l2;
    } else if current_l1.is_some() && current_l2.is_none() {
        return current_l1;
    }

    let mut result_head = Box::new(ListNode::new(0));
    let mut current_head = &mut result_head;
    let mut count = 0;
	while current_l1.is_some() && current_l2.is_some() {
		count = count + 1;
        let mut current1_up = current_l1.take().unwrap();
        let mut current2_up = current_l2.take().unwrap();
        println!("{} current l1: {:?}", count, current1_up);
        println!("{} current l2: {:?}", count, current2_up);
        if current1_up.val <= current2_up.val {
            current_l1 = current1_up.next.take();
			current_head.next = Some(current1_up);
			current_l2 = Some(current2_up);
        } else {
			current_l2 = current2_up.next.take();
			current_head.next = Some(current2_up);
			current_l1 = Some(current1_up);

        }
		println!("{} new current l1: {:?}", count, current_l1);
		println!("{} new current l2: {:?}", count, current_l2);
        current_head = current_head.next.as_mut().unwrap()
    }

	if current_l1.is_some() {
		current_head.next = current_l1;
	} else if current_l2.is_some() {
		current_head.next = current_l2;

	}
	return result_head.next; 
	
}
