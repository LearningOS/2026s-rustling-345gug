/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }
    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => {
                // 树为空，直接新建根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(node) => {
                // 根节点存在，调用TreeNode的insert递归插入
                node.insert(value);
            }
        }
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        self.search_recursive(&self.root, value)
    }
    // 辅助递归查找函数
    fn search_recursive(&self, node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => self.search_recursive(&n.left, value),
                Ordering::Greater => self.search_recursive(&n.right, value),
                Ordering::Equal => true,
            },
        }
    }
}
impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 待插入值更小 → 往左子树
                match &mut self.left {
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(left_node) => {
                        left_node.insert(value);
                    }
                }
            }
            Ordering::Greater => {
                // 待插入值更大 → 往右子树
                match &mut self.right {
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(right_node) => {
                        right_node.insert(value);
                    }
                }
            }
            Ordering::Equal => {
                // 相等：重复值，什么都不做，不插入（匹配test_insert_duplicate）
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
