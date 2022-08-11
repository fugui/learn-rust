/**
 * Create a linked list is not easy.
 * https://rust-unofficial.github.io/too-many-lists/index.html
 */

#[allow(dead_code)]
struct Node {
    value: i32,
    previous: Option<Box<Node>>,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value: value,
            previous: None,
            next: None,
        }
    }

    fn append(&mut self, _value: i32) -> &mut Node {
        return self;
    }
}

fn main() {
    let facts = [1, 3, 4, 6, 12, 53, 27, 34];

    let mut head = Node::new(facts[0]);

    head.value += 1;

    head.append(facts[1]);
}
