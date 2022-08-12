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
    let mut root = Link::Tail { item: 20 };
    root.push(50);
    root.push(60);
    root.traverse();
}
