#[derive(PartialEq, Clone, Eq, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
///创建链表
fn create_link(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for value in values.iter().rev() {
        let mut new_node = Box::new(ListNode::new(*value));

        new_node.next = head;

        head = Some(new_node)
    }

    head
}
///打印链表
fn print_link(head: &Option<Box<ListNode>>) {
    let mut list_node = Vec::new();
    let mut current = head.as_ref();
    loop {
        match current {
            Some(node) => {
                list_node.push(node.val);
                current = node.next.as_ref()
            }
            None => {
                break;
            }
        }
    }
    println!("{:?}", list_node);
}
///计算最大公约数
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}
// fn inset(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     if head.is_none() {
//         return head;
//     }
//     let mut mutable = head;
//     let mut  current = mutable.as_mut();

//     while let Some(current_node) = current {
//         if let Some(next_node) = current_node.next.take() {
//             let mut new_node = Box::new(ListNode::new(gcd(current_node.val, next_node.val)));

//             new_node.next = Some(next_node);

//             current_node.next = Some(new_node);

//             current = current_node.next.as_mut().unwrap().next.as_mut();
//         }else {
//             break;
//         }
//     }

//     return mutable;
// }

/// 请在在一个链表中每相邻的两个节点间插入一个结点, 插入节点的 val 为两个节点 val 的最大公约数
/// 例如： [10, 5, 6] -> [10, 5, 5, 1, 6]
/// 复杂度要求： O(n)/O(1)
fn insert_great_common_divisor(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut mutable_head = head;
    let mut current = mutable_head.as_mut();

    while let Some(current_node) = current {
        // take只在堆Option的可变借用上使用
        if let Some(next_node) = current_node.next.take() {
            let mut new_node = Box::new(ListNode::new(gcd(current_node.val, next_node.val)));
            new_node.next = Some(next_node);
            current_node.next = Some(new_node);
            current = current_node.next.as_mut().unwrap().next.as_mut();
        } else {
            break;
        }
    }
    return mutable_head;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let values = vec![10, 20, 30];
        let list_node = create_link(values);

        let list_node_inserted = insert_great_common_divisor(list_node);

        print_link(&list_node_inserted);
    }
}
