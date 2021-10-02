// Binary tree using memory arena
//
// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

// implicitly first node is top node
#[derive(Debug)]
pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,

    pub value: T,
}

impl<T> Tree<T> {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(index)
    }

    pub fn new_node(&mut self, value: T) -> usize {
        let node = Node {
            parent: None,
            left: None,
            right: None,
            value: value,
        };
        self.nodes.push(node);
        self.nodes.len()
    }

    pub fn left(&self, node: &Node<T>) -> Option<&Node<T>> {
        if let Some(left_id) = node.left {
            self.nodes.get(left_id)
        } else {
            None
        }
    }

    pub fn right(&self, node: &Node<T>) -> Option<&Node<T>> {
        if let Some(right_id) = node.right {
            self.nodes.get(right_id)
        } else {
            None
        }
    }

    pub fn parent(&self, node: &Node<T>) -> Option<&Node<T>> {
        if let Some(parent_id) = node.parent {
            self.nodes.get(parent_id)
        } else {
            None
        }
    }

    pub fn set_left(&mut self, parent_id: usize, value: T) -> Option<usize> {
        if self.nodes.len() >= parent_id {
            return None;
        }
        let child_id = self.new_node(value);
        let mut parent = self.get_mut(parent_id).unwrap();
        parent.left = Some(child_id);
        let mut child = self.get_mut(child_id).unwrap();
        child.parent = Some(parent_id);

        return Some(child_id);
    }

    pub fn set_right(&mut self, parent_id: usize, value: T) -> Option<usize> {
        if self.nodes.len() >= parent_id {
            return None;
        }
        let child_id = self.new_node(value);
        let mut parent = self.get_mut(parent_id).unwrap();
        parent.right = Some(child_id);
        let mut child = self.get_mut(child_id).unwrap();
        child.parent = Some(parent_id);

        return Some(child_id);
    }

    pub fn post_trav<F>(tree_cell: Rc<RefCell<Tree<T>>>, mut f: F)
    where
        F: FnMut(usize),
    {
        if tree_cell.borrow().len() > 0 {
            Tree::post_trav_inner(tree_cell.clone(), 0, &mut f);
        }
    }

    fn post_trav_inner<F>(tree_cell: Rc<RefCell<Tree<T>>>, idx: usize, f: &mut F)
    where
        F: FnMut(usize),
    {
        let mut left_idx: usize = 0;
        let mut right_idx: usize = 0;

        {
            let cell_ref = tree_cell.borrow();
            if let Some(node) = cell_ref.get(idx) {
                if cell_ref.left(node).is_some() {
                    left_idx = node.left.unwrap();
                }
                if cell_ref.right(node).is_some() {
                    right_idx = node.right.unwrap();
                }
            }
        }

        if left_idx > 0 {
            Tree::post_trav_inner(tree_cell.clone(), left_idx, f);
        }

        if right_idx > 0 {
            Tree::post_trav_inner(tree_cell.clone(), right_idx, f);
        }

        f(idx);
    }
}

impl<T> Tree<T> {
    // builder get zero-start node index
    pub fn new_complete<F>(depth: usize, builder: F) -> Self
    where
        F: Fn(usize) -> T,
    {
        let node_n = 2usize.pow((depth + 1) as u32);
        let mut nodes: Vec<Node<T>> = Vec::with_capacity(node_n - 1);

        for i in 1..node_n {
            let parent = if i == 1 { None } else { Some(i / 2 - 1) };
            let left = if i >= node_n / 2 {
                None
            } else {
                Some(i * 2 - 1)
            };
            let right = if i >= node_n / 2 { None } else { Some(i * 2) };

            nodes.push(Node {
                parent,
                left,
                right,
                value: builder(i - 1),
            });
        }

        Self { nodes }
    }

    pub fn set_new_complete<F>(&mut self, depth: usize, builder: F) -> ()
    where
        F: Fn(usize) -> T,
    {
        let complete = Tree::<T>::new_complete(depth, builder);
        self.nodes = complete.nodes;
    }
}

impl<T> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
