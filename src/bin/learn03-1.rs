use std::fmt::Display;

/**
 * Linked list with enum
 * https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676
 */

enum LinkedList<T> {
    None,
    Tail { item: T },
    Link { item: T, next: Box<LinkedList<T>> },
}

impl<T> LinkedList<T>
where
    T: Copy,
    T: Display,
{
    pub fn push(&mut self, value: T) {
        match self {
            LinkedList::None => *self = LinkedList::Tail { item: value },
            LinkedList::Tail { item } => {
                *self = LinkedList::Link {
                    item: *item,
                    next: Box::new(LinkedList::Tail { item: value }),
                }
            }
            LinkedList::Link { next, .. } => next.push(value),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        return match self {
            LinkedList::None => Option::None,
            LinkedList::Tail { item } => {
                let ret = Some(*item);
                *self = LinkedList::None;
                ret
            }
            LinkedList::Link { next, item: value } => {
                if let LinkedList::Tail { item } = next.as_ref() {
                    let ret = Some(*item);
                    *self = LinkedList::Tail { item: *value };
                    ret
                } else {
                    next.pop()
                }
            }
        };
    }

    pub fn traverse(&self) {
        match self {
            LinkedList::None => return,
            LinkedList::Tail { item } => println!("->{}", item),
            LinkedList::Link { item, next } => {
                println!("->{}", item);
                next.traverse()
            }
        }
    }
}

fn main() {
    let mut root = LinkedList::Tail { item: "Great" };
    root.push("50");
    root.push("60");
    println!("Pop {}", root.pop().unwrap());
    println!("Pop {}", root.pop().unwrap());
    root.push("China");
    root.traverse();
}
