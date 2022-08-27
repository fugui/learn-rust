use std::fmt::Display;

/**
 * From document : https://rust-unofficial.github.io/too-many-lists/second.html
 */
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct LinkedList<T>
where
    T: Display,
    T: Copy,
{
    head: Link<T>,
}

impl<T> LinkedList<T>
where
    T: Display,
    T: Copy,
{
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value: value,
            next: self.head.take(), //std::mem::replace(&mut self.head, None),
        }))
    }

    fn pop(&mut self) -> Option<T> {
        /*match self.head.take() {
            //std::mem::replace(&mut self.head, None) {
            Option::None => None,
            Option::Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }*/
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

impl<T> Drop for LinkedList<T>
where
    T: Display,
    T: Copy,
{
    fn drop(&mut self) {
        let mut current = self.head.take(); //std::mem::replace(&mut self.head, None);
        while let Some(mut node) = current {
            println!("Deleting {}", node.value);
            current = node.next.take(); //std::mem::replace(&mut node.next, None)
        }
    }
}

#[test]
fn test_peek() {
    let mut list = LinkedList::new();
    assert_eq!(list.peek(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
}

impl<T> Iterator for LinkedList<T>
where
    T: Display,
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[test]
fn test_iterator() {
    let mut list = LinkedList::new();
    list.push("A");
    list.push("B");
    list.push("C");

    assert_eq!(list.next(), Some("C"));

    while let Some(value) = list.next() {
        println!("{}", value);
    }
    assert_eq!(list.next(), None);
}

fn main() {
    let mut list = LinkedList::new();
    list.push(10);
    list.push(20);
    list.pop();
    list.push(30);
    println!("Peek value: {:?}", list.peek());
    println!("{:?}", list);
    drop(list);
}
