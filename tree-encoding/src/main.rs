// pub struct BinaryTree<T> {
//     head: Option<Node<T>>,
// }
// impl<T> BinaryTree<T> {
//     fn new(head: Node<T>) -> BinaryTree<T> {
//         BinaryTree { head: Some(head) }
//     }
// }

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use]
extern crate serde_derive;

use std::fs::File;

type ChildNode<T> = Option<Box<Node<T>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node<T> {
    /// The actual data which will be stored within the tree
    pub data: T,
    left_child: ChildNode<T>,
    right_child: ChildNode<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedTree<T> {
    data_vec: Vec<T>,
    is_node_vec: Vec<bool>,
}

impl<T> EncodedTree<T> {
    fn new(data_vec: Vec<T>, is_node_vec: Vec<bool>) -> EncodedTree<T> {
        EncodedTree {
            data_vec,
            is_node_vec,
        }
    }
}

impl<T> Node<T>
where
    T: std::fmt::Debug + Copy + serde::Serialize,
{
    fn new(data: T) -> Node<T> {
        Node {
            left_child: None,
            right_child: None,
            data,
        }
    }
    pub fn add_right_child(&mut self, data: T) {
        let child = Node::new(data);
        self.right_child = Some(Box::new(child));
    }
    pub fn add_left_child(&mut self, data: T) {
        let child = Node::new(data);
        self.left_child = Some(Box::new(child));
    }

    pub fn print_tree(&self) {
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
    pub fn encode_tree(&self) -> EncodedTree<T> {
        let mut data_vec: Vec<T> = vec![self.data];
        let mut is_node_vec: Vec<bool> = vec![true];
        self._encode_tree(&mut data_vec, &mut is_node_vec);

        EncodedTree {
            data_vec,
            is_node_vec,
        }
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

    fn to_json(&self) -> serde_json::Result<String> {
        let encoded_tree = self.encode_tree();

        serde_json::to_string(&encoded_tree)
    }

    fn save_tree(&self, filename: &str, pretty: bool) -> serde_json::Result<()> {
        let encoded_tree = self.encode_tree();
        // let json = self.to_json()?;

        if pretty {
            serde_json::to_writer_pretty(&File::create(filename).unwrap(), &encoded_tree)
        } else {
            serde_json::to_writer(&File::create(filename).unwrap(), &encoded_tree)
        }
    }
}

fn main() {
    let mut root: Node<i32> = Node::new(10);
    root.add_left_child(20);
    root.add_right_child(30);

    root.left_child.as_mut().unwrap().add_left_child(40);
    root.left_child.as_mut().unwrap().add_right_child(50);
    root.right_child.as_mut().unwrap().add_right_child(70);
    // root.print_tree();
    let encoded = root.to_json();
    println!("Data: {:?}", encoded);
    let res = root.save_tree("tree.json", true);
    println!("res: {:?}", res);
}
