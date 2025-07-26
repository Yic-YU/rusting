use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}
impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}
struct DoublyLink<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}
impl<T: Clone + std::fmt::Debug> DoublyLink<T> {
    ///创建一个空链表
    fn new() -> Self {
        DoublyLink {
            head: None,
            tail: None,
            len: 0,
        }
    }
    /// 返回链表长度
    fn len(&self) -> usize {
        self.len
    }
    /// 检查链表是否为空
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    ///增
    fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_node);
            }

            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node)
            }
        }
        self.len += 1;
    }

    fn get(&self, index: usize) -> Option<T> {
        self.get_node(index).map(|node| node.borrow().value.clone())
    }
    ///查（index）
    fn get_node(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if index >= self.len {
            return None;
        }

        let mut current_node = self.head.clone();
        for _ in 0..index {
            let next = current_node.as_ref()?.borrow().next.clone();
            current_node = next;
        }

        current_node
    }
    ///删
    fn remove(&mut self, index: usize) -> Option<T> {
        let remove_node = self.get_node(index).unwrap();
        let prev_node_opt = remove_node.borrow().prev.as_ref().and_then(|p| p.upgrade());
        let next_node_opt = remove_node.borrow().next.clone();
        match (prev_node_opt, next_node_opt) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(Rc::downgrade(&prev));
            }
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            }
            (Some(prev), None) => {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            }
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
        }
        self.len -= 1;

        let value = Rc::try_unwrap(remove_node)
            .ok()
            .expect("Failed to unwrap Rc, something is wrong")
            .into_inner()
            .value;

        Some(value)
    }

    pub fn print_list(&self) {
        let mut current = self.head.clone();
        print!("List (len={}): [", self.len);
        while let Some(node) = current {
            print!("{:?}", node.borrow().value);
            current = node.borrow().next.clone();
            if current.is_some() {
                print!(" <-> ");
            }
        }
        println!("]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doublylink() {
        let mut list = DoublyLink::<i32>::new();
        list.print_list(); // List (len=0): []

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(40);
        list.print_list(); // List (len=4): [10 <-> 20 <-> 30 <-> 40]
        println!("链表长度： {}", list.len()); // 4

        println!("index0:元素为{:?}", list.get(0)); // Some(10)
        println!("index2:元素为{:?}", list.get(2)); // Some(30)
        println!("index3:元素为{:?}", list.get(3)); // Some(40)
        println!("index5:元素为{:?}", list.get(5)); // None (out of bounds)

        println!("删除index为1的元素");
        let removed_val = list.remove(1);
        println!("删除的值为: {:?}", removed_val); // Some(20)
        list.print_list(); // List (len=3): [10 <-> 30 <-> 40]
    }
}
