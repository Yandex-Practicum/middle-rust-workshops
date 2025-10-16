struct SingleLinkedList<T> {
    len: usize,
    head: Option<Box<SingleLinkedListNode<T>>>,
}
struct SingleLinkedListNode<T> {
    value: T,
    next: Option<Box<SingleLinkedListNode<T>>>,
}
impl<T> SingleLinkedList<T>
where T: std::fmt::Display,
{
    fn new () -> Self {
        Self{len: 0, head: None}
    }
    fn push_back (&mut self, value: T) {
        let new_node = Box::new(SingleLinkedListNode{value, next: None});
        let mut current = &mut self.head;
        //while let Some(current_some) = &mut current { // cannot borrow `current_some` as mutable because it is also borrowed as immutable
        while let Some(current_some) = current {
            current = &mut current_some.next;
        }
        *current = Some(new_node);
        self.len += 1;
    }
    fn push_front (&mut self, value: T) {
        //let new_node = Box::new(SingleLinkedListNode{value, next: self.head}); // cannot move out of `self.head` which is behind a mutable reference
        let new_node = Box::new(SingleLinkedListNode{value, next: self.head.take()});
        self.head = Some(new_node);
        self.len += 1;
    }
    fn to_string (&self) -> String {
        use std::fmt::Write;
        let mut result = format!("{}:{{", self.len);
        let mut current = &self.head;
        while let Some(current_some) = current {
            write!(&mut result, "{}, ", current_some.value).unwrap();
            current = &current_some.next;
        }
        write!(&mut result, "}}").unwrap();
        result
    }
    //fn pop_back(&self) -> T { // лучше возвращать Option, чтобы не паниковаьт
    fn pop_back (&mut self) -> Option<T> {
        ////let Some(mut cursor) = &mut self.head else { // в while будет expected Box<> found &mut Box<>
        //let Some(mut cursor) = self.head.as_mut() else {
        //    return None;
        //};
        //if cursor.next.is_none() {
        //    return self.head.take().map(|boxed| boxed.value);
        //}
        ////while let Some(next) = &mut cursor.next {
        ////    cursor = next;
        ////}
        ////cursor.next.take().map(|boxed| boxed.value) // cannot borrow `cursor.next` as mutable more than once
        if self.head.is_none() {
            return None;
        }
        self.len -= 1;
        if self.head.as_mut().unwrap().next.is_none() {
            let value = self.head.take().unwrap().value;
            self.head = None;
            return Some(value);
        }
        let mut cursor = &mut self.head;
        while let Some(cursor_some) = cursor {
            if cursor_some.next.as_ref().unwrap().next.is_none() {
                let value = cursor_some.next.take().unwrap().value;
                cursor_some.next = None;
                return Some(value);
            }
            cursor = &mut cursor_some.next;
        }
        unreachable!("logic error - the emptiness must be already checked")
    }
}

fn single_linked_list_case() {
    let mut list = SingleLinkedList::new();
    list.push_back(1);
    list.push_front(0);
    println!("single-list pushed: {}", list.to_string());
    list.pop_back();
    println!("single-list poped: {}", list.to_string());
}

//DoublyLinkedList без rc и weak нереализуем, тк Box владеет своими данными
//struct DoublyLinkedList<T> {
//    len: usize,
//    head: Option<Box<SingleLinkedListNode<T>>>,
//}
//struct DoublyLinkedListNode<T> {
//    value: T,
//    next: Option<Box<SingleLinkedListNode<T>>>,
//}
struct DoublyLinkedList<T> {
    len: usize,
    head: Option<std::rc::Rc<std::cell::RefCell<DoublyLinkedListNode<T>>>>,
    tail: Option<std::rc::Weak<std::cell::RefCell<DoublyLinkedListNode<T>>>>,
}
struct DoublyLinkedListNode<T> {
    value: T,
    next: Option<std::rc::Rc<std::cell::RefCell<DoublyLinkedListNode<T>>>>,
    prev: Option<std::rc::Weak<std::cell::RefCell<DoublyLinkedListNode<T>>>>,
}
impl<T> DoublyLinkedList<T>
where T: std::fmt::Display,
{
    fn new () -> Self {
        Self{len: 0, head: None, tail: None}
    }
    fn push_back (&mut self, value: T) {
        let new_node = std::rc::Rc::new(std::cell::RefCell::new(DoublyLinkedListNode{value, next: None, prev: None}));
        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(std::rc::Rc::downgrade(&new_node));
        }
        else {
            let mut current = self.head.as_ref().unwrap().clone();
            while current.borrow().next.is_some() {
                let next = current.borrow().next.as_ref().unwrap().clone();
                current = next;
            }
            new_node.borrow_mut().prev = Some(std::rc::Rc::downgrade(&current));
            current.borrow_mut().next = Some(new_node);
        }
        self.len += 1;
    }
    fn to_string (&self) -> String {
        use std::fmt::Write;
        let mut result = format!("{}:{{", self.len);
        let mut current = self.head.clone();
        while let Some(current_some) = current {
            write!(&mut result, "{}, ", current_some.borrow().value).unwrap();
            current = current_some.borrow().next.clone();
        }
        write!(&mut result, "}}").unwrap();
        result
    }
    fn pop_back (&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        self.len -= 1;
        if self.head.clone().unwrap().borrow().next.is_none() {
            let value = self.head.take().map(|refcell| std::rc::Rc::into_inner(refcell).unwrap().into_inner().value);
            self.head = None;
            self.tail = None;
            return value;
        }

        let mut cursor = self.head.clone().unwrap();
        while cursor.borrow().next.as_ref().unwrap().borrow().next.is_some() {
            let next = cursor.borrow().next.clone().unwrap();
            cursor = next;
        }
        let next = std::rc::Rc::into_inner(cursor.borrow_mut().next.take().unwrap()).unwrap();
        Some(next.into_inner().value)
    }
}

fn doubly_linked_list_case() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    println!("single-list pushed: {}", list.to_string());
    list.pop_back();
    list.pop_back();
    println!("single-list poped: {}", list.to_string());
}

fn main() {
    println!("Hello, world!");
    single_linked_list_case();
    doubly_linked_list_case();
}
