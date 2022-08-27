/**
 * From document : https://rust-unofficial.github.io/too-many-lists/second.html
 */
#[derive(Debug)]
struct Node {
    value: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: i32) {
        self.head = Some(Box::new(Node {
            value: value,
            next: std::mem::replace(&mut self.head, None),
        }))
    }

    fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, None) {
            Option::None => None,
            Option::Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(10);
    list.push(20);
    list.pop();
    list.push(30);
    println!("{:?}", list);
}
