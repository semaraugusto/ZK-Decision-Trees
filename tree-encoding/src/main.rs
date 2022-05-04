type ChildNode<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    left_child: ChildNode<T>,
    right_child: ChildNode<T>,
    /// The actual data which will be stored within the tree
    pub data: T,
}

pub struct BinaryTree<T> {
    head: Option<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            left_child: None,
            right_child: None,
            data,
        }
    }
    fn add_right_child(&mut self, data: T) {
        let child = Node::new(data);
        self.right_child = Some(Box::new(child));
    }
    fn add_left_child(&mut self, data: T) {
        let child = Node::new(data);
        self.left_child = Some(Box::new(child));
    }
}

impl<T> BinaryTree<T> {
    fn new(head: Node<T>) -> BinaryTree<T> {
        BinaryTree { head: Some(head) }
    }
}

fn main() {
    let mut root = Node::new(10);
    root.add_left_child(20);
    root.add_right_child(30);

    root.left_child.as_mut().unwrap().add_right_child(40);
    root.left_child.as_mut().unwrap().add_left_child(50);
    root.right_child.as_mut().unwrap().add_right_child(60);
}
