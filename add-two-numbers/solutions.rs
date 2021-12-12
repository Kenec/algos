struct Solution;

#[derive(Debug, Clone, Eq, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode{ val, next: None }
    }
}

fn to_list(vec: Vec<i32>) -> List {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(l1: List, l2: List) -> List {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0;
        let mut result = None;
        let mut cur = &mut result;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;

                println!("Sum L1: {:?}", sum);
                println!("L1 Node {:?}", l1);
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next;

                println!("Sum L2: {:?}", sum);
                println!("L2 Node {:?}", l2);
            }

            carry = sum / 10;
            println!("Carry {:?}", carry);

            *cur = Some(Box::new(ListNode::new(sum % 10)));
            println!("*cur {:?}", *cur);

            cur = &mut cur.as_mut().unwrap().next;
            println!("cur {:?} ", cur);

            println!("=========================================");
        }

        result
    }
}

fn main() {
    let l1 = vec![2,4,3];
    let l2 = vec![5,6,4];

    let solution = Solution::add_two_numbers(to_list(l1), to_list(l2));
}
