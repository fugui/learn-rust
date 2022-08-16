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
    T: PartialEq,
{
    pub fn push(&mut self, value: T) {
        match self {
            Self::None => *self = Self::Tail { item: value },
            Self::Tail { item } => {
                *self = Self::Link {
                    item: *item,
                    next: Box::new(Self::Tail { item: value }),
                }
            }
            Self::Link { next, .. } => next.push(value),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        return match self {
            Self::None => Option::None,
            Self::Tail { item } => {
                let ret = Some(*item);
                *self = Self::None;
                ret
            }
            Self::Link { next, item: value } => {
                if let Self::Tail { item } = next.as_ref() {
                    let ret = Some(*item);
                    *self = Self::Tail { item: *value };
                    ret
                } else {
                    next.pop()
                }
            }
        };
    }

    pub fn delete(&mut self, value: T) {
        match self {
            Self::Tail { item } if *item == value => *self = Self::None,
            Self::Link { item, next } if *item != value => next.delete(value),
            Self::Link { item, next } if *item == value => {
                if let Self::Tail { item } = next.as_mut() {
                    *self = Self::Tail { item: *item }
                } else if let Self::Link { item, next: n } = next.as_mut() {
                    *self = Self::Link {
                        item: *item,
                        next: std::mem::replace(n, Box::new(Self::None)),
                    }
                }
            }
            _ => return,
        }
    }

    pub fn traverse(&self) {
        match self {
            Self::None => return,
            Self::Tail { item } => println!("->{}", item),
            Self::Link { item, next } => {
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
    root.push("70");
    root.push("80");
    root.push("90");
    root.push("100");
    root.delete("70");
    root.delete("100");
    root.delete("90");
    root.traverse();
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basic_test_list() {
        let mut root = LinkedList::Tail { item: "Great" };
        assert_eq!(root.pop(), Some("Great"));
    }
}
