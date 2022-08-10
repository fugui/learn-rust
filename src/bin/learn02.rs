use std::fmt::Display;

struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Root: {}", self.value)?;
        write!(
            f,
            " Left: {}",
            match &self.left {
                Some(node) => node.value.to_string(),
                None => "None".to_string(),
            }
        )?;
        writeln!(
            f,
            " Right: {}",
            match &self.right {
                Some(node) => node.value.to_string(),
                None => "None".to_string(),
            }
        )?;

        if let Some(left) = &self.left {
            write!(f, "{}", left.as_ref())?;
        }

        if let Some(right) = &self.right {
            write!(f, "{}", right.as_ref())?;
        }

        Result::Ok(())
    }
}

fn insert_node(root: &mut TreeNode, value: i32) {
    if value < root.value {
        if let Some(_) = root.left {
            insert_node(root.left.as_mut().unwrap(), value)
        } else {
            root.left = Some(Box::new(TreeNode {
                value: value,
                left: None,
                right: None,
            }))
        }
    } else if value > root.value {
        match root.right {
            Some(_) => insert_node(root.right.as_mut().unwrap(), value),
            None => {
                root.right = Some(Box::new(TreeNode {
                    value: value,
                    left: None,
                    right: None,
                }))
            }
        }
    }
}

fn main() {
    let facts = [1, 3, 4, 6, 12, 53, 27, 34];

    let mut root = TreeNode {
        value: 15,
        left: None,
        right: None,
    };

    for i in facts {
        insert_node(&mut root, i);
    }

    print!("{}", root);
}
