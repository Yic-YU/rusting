use std::{iter::Rev, sync::Arc};

#[derive(Clone, Debug)]
pub struct Stack<T> {
    elem: Vec<T>,
}
impl<T> Stack<T> {
    ///创建一个空栈
    pub fn new() -> Self {
        Stack { elem: Vec::new() }
    }
    ///将元素压入栈顶
    pub fn push(&mut self, elem: T) {
        self.elem.push(elem);
    }
    ///从栈顶弹出一个元素
    pub fn pop(&mut self) -> Option<T> {
        self.elem.pop()
    }
    ///栈顶到栈底的迭代
    pub fn iter(&self) -> Rev<std::slice::Iter<'_, T>> {
        self.elem.iter().rev()
    }
    ///栈顶到栈底的迭代
    pub fn into_iter(self) -> Rev<std::vec::IntoIter<T>> {
        self.elem.into_iter().rev()
    }
    ///返回栈顶元素
    pub fn top(&self) -> Option<&T> {
        self.elem.last()
    }
}

macro_rules! stack {
    ($($elem:expr),*) => {
        {
            let mut stack = Stack::new();
            $(
                stack.push($elem);
            )*
            stack
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        // let mut stack = stack![1,2,34,56];
        // stack.push(100);
        // if let Some(value) = stack.top() {
        //     println!("{:?}",value);
        // }
        // stack.pop();
        // println!("{:?}",stack.iter());
        // let new_stack = stack.into_iter();
        // println!("{:?}",new_stack);
    }
}
