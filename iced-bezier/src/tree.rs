// Binary tree using memory arena
//
// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

use std::ops::Deref;

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

#[derive(Debug)]
struct TreePostIterMut<'a, T> {
    tree: &'a mut Tree<T>,
    idx: usize,
    stopped: bool,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

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

    pub fn leftest_idx(&self, index: usize) -> usize {
        let mut idx = index;
        let mut node = self.get(idx).unwrap();

        loop {
            if let Some(left_id) = node.left {
                if let Some(left) = self.get(left_id) {
                    idx = left_id;
                    node = left;
                    continue;
                }
            }

            if let Some(right_id) = node.right {
                if let Some(right) = self.get(right_id) {
                    idx = right_id;
                    node = right;
                    continue;
                }
            }

            break;
        }

        idx
    }

    pub fn post_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Node<T>> {
        TreePostIterMut::new(self)
    }
}

impl<'a, T> TreePostIterMut<'a, T> {
    fn new(tree: &'a mut Tree<T>) -> Self {
        if tree.nodes.len() == 0 {
            return TreePostIterMut {
                tree,
                idx: 0,
                stopped: true,
            };
        }
        let mut idx: usize = tree.leftest_idx(0);
        TreePostIterMut {
            tree,
            idx,
            stopped: false,
        }
    }
}

impl<'a, T: 'a> Iterator for TreePostIterMut<'a, T> {
    type Item = &'a mut Node<T>;
    // explicit iterator
    fn next(self: &'_ mut TreePostIterMut<'a, T>) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }

        let curr_node_opt = self.tree.nodes.get_mut(self.idx);

        if curr_node_opt.is_none() {
            return None;
        }

        let curr_node = curr_node_opt?;
        if let Some(parent) = self.tree.parent(curr_node) {
            if parent.left.unwrap() == self.idx {
                // come from left -> go to right
                if self.tree.right(parent).is_some() {
                    self.idx = self.tree.leftest_idx(parent.right?);
                } else {
                    self.idx = curr_node.parent?;
                }
            } else {
                // come from right -> remain parent
                self.idx = curr_node.parent?;
            }
        } else {
            self.stopped = true;
        }

        curr_node_opt
    }
}

impl<T: Default> Tree<T> {
    pub fn new_complete(depth: usize) -> Self {
        let node_n = 2usize.pow((depth + 1) as u32);
        let mut nodes: Vec<Node<T>> = Vec::with_capacity(node_n);

        for i in 1..=node_n {
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
                value: Default::default(),
            });
        }

        Self { nodes }
    }

    pub fn set_new_complete(&mut self, depth: usize) -> () {
        let complete = Tree::<T>::new_complete(depth);
        self.nodes = complete.nodes;
    }
}

impl<T> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
