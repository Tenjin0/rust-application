#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}


impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

}



fn inner_merge_two_lists_rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
   
  todo!();
}



pub fn create_min(p1: Option<Box<ListNode>>, p2: Option<Box<ListNode>>) -> ListNode {
  let x: Box<ListNode> = p1.unwrap();
  let y = p2.unwrap();
  let start : ListNode;
  if x.val < y.val {
    let val = x.val;
    start = ListNode::new(val)
  } else {
    let val = y.val;
    start = ListNode::new(val)
  }
  return start;
}

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

  if l1 == None && l2 == None {
    return None
  }
  if l1 == None && l2 != None {
    return l1
  }

  if l1 != None && l2 == None {
    return l2
  }

  let mut result: Option<Box<ListNode>>;
  let mut tail: &Option<Box<ListNode>>;

  let p1: Option<Box<ListNode>> = l1;
  let p2: Option<Box<ListNode>> = l2;
  
  // {
    // let start = create_min(p1, p2);
    // tail = &start.next;
    // result = Some(Box::new(start));
  // }

  loop {
    let mut next : Option<Box<ListNode>> = None;
   
    break;
  }
  todo!();
}
