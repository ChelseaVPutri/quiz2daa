use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        //TODO
        //default if current node is NIL
        let mut current = self.get_bst_nodelink_copy();

        while current.borrow().key.is_some() && current.borrow().key.unwrap() != *value {
            let next_node = {
                let current_ref = current.borrow();
                if *value < current_ref.key.unwrap() {
                    current_ref.left.clone()
                } else {
                    current_ref.right.clone()
                }
            };

            match next_node {
                None => return None,
                Some(next) => current = next,
            }
        }

        if current.borrow().key.is_some() {
            Some(current)
        } else {
            None
        }

    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        //TODO
        let mut current = self.get_bst_nodelink_copy();
        
        loop {
            let next_node = {
                let current_ref = current.borrow();
                current_ref.left.clone()
            };

            match next_node {
                None => break,
                Some(left) => current = left,
            }
        }
        current
    }

    pub fn maximum(&self) -> BstNodeLink {
        //TODO
        let mut current = self.get_bst_nodelink_copy();
        
        loop {
            let next_node = {
                let current_ref = current.borrow();
                current_ref.right.clone()
            };

            match next_node {
                None => break,
                Some(right) => current = right,
            }
        }
        current
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * Find node successor according to the book
     * Possible to return self, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> BstNodeLink {
        //TODO
        if let Some(right_child) = x_node.borrow().right.clone() {
            return right_child.borrow().minimum();
        }

        let mut current = x_node.clone();
        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());

        while y_node.is_some() && BstNode::is_node_match(&current, &y_node.as_ref().unwrap().borrow().right.clone().unwrap()) {
            current = y_node.unwrap();
            y_node = BstNode::upgrade_weak_to_strong(current.borrow().parent.clone());
        }

        match y_node {
            None => x_node.clone(),
            Some(parent) => parent,
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
