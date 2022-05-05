type ChildNode<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    /// The actual data which will be stored within the tree
    pub data: T,
    left_child: ChildNode<T>,
    right_child: ChildNode<T>,
}

pub struct BinaryTree<T> {
    head: Option<Node<T>>,
}

impl<T> Node<T>
where
    T: std::fmt::Debug + Copy,
{
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

    fn print_tree(&self) {
        println!("Data: {:?}", self.data);
        if let Some(child) = &self.left_child {
            print!("Left: ");
            child.print_tree();
        }
        if let Some(child) = &self.right_child {
            print!("Right: ");
            child.print_tree();
        }
    }
    fn encode_tree(&self) -> (Vec<T>, Vec<bool>) {
        let mut data_vec: Vec<T> = vec![self.data];
        let mut is_node_vec: Vec<bool> = vec![true];
        self._encode_tree(&mut data_vec, &mut is_node_vec);

        (data_vec, is_node_vec)
    }

    fn _encode_tree(&self, data_vec: &mut Vec<T>, is_node_vec: &mut Vec<bool>) {
        match &self.left_child {
            Some(child) => {
                data_vec.push(child.data);
                is_node_vec.push(true);
                child._encode_tree(data_vec, is_node_vec);
            }
            None => {
                is_node_vec.push(false);
            }
        }
        match &self.right_child {
            Some(child) => {
                data_vec.push(child.data);
                is_node_vec.push(true);
                child._encode_tree(data_vec, is_node_vec);
            }
            None => {
                is_node_vec.push(false);
            }
        }
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

    root.left_child.as_mut().unwrap().add_left_child(40);
    root.left_child.as_mut().unwrap().add_right_child(50);
    root.right_child.as_mut().unwrap().add_right_child(70);
    // root.print_tree();
    let (data_vec, is_node_vec) = root.encode_tree();
    println!("Data: {:?}", data_vec);
    println!("node_vec: {:?}", is_node_vec);
}
