use std::fmt::Display;

/**
 * Linked list with enum
 * https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676
 */

enum Link<T> {
    None,
    Tail { item: T },
    Link { item: T, next: Box<Link<T>> },
}

impl<T> Link<T>
where
    T: Copy,
    T: Display,
{
    pub fn push(&mut self, value: T) {
        match self {
            Link::None => *self = Link::Tail { item: value },
            Link::Tail { item } => {
                *self = Link::Link {
                    item: *item,
                    next: Box::new(Link::Tail { item: value }),
                }
            }
            Link::Link { next, .. } => next.push(value),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        return match self {
            Link::None => Option::None,
            Link::Tail { item } => {
                let ret = Some(*item);
                *self = Link::None;
                ret
            }
            Link::Link { next, item: value } => {
                if let Link::Tail { item } = next.as_ref() {
                    let ret = Some(*item);
                    *self = Link::Tail { item: *value };
                    ret
                } else {
                    next.pop()
                }
            }
        };
    }

    pub fn traverse(&self) {
        match self {
            Link::None => return,
            Link::Tail { item } => println!("->{}", item),
            Link::Link { item, next } => {
                println!("->{}", item);
                next.traverse()
            }
        }
    }
}

fn main() {
    let mut root = Link::Tail { item: "Great" };
    root.push("50");
    root.push("60");
    println!("Pop {}", root.pop().unwrap());
    println!("Pop {}", root.pop().unwrap());
    root.push("China");
    root.traverse();
}
