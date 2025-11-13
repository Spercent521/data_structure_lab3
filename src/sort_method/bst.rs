use std::time::Instant;
use colored::Colorize;

/// 定义平衡二叉树节点
struct TreeNode {
    val: i32,
    origin_pos_in_sequence: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32 , origin_pos_in_sequence: i32) -> Self {
        TreeNode { val, origin_pos_in_sequence, left: None, right: None }
    }
}

/// 定义二叉排序树(root)
pub struct BST {
    root: Option<Box<TreeNode>>,
}

impl BST {
    // new a BST
    pub fn new() -> Self {
        BST { root: None }
    }

    // insert value
    pub fn insert(&mut self, val: i32, origin_pos: i32) {
        let new_node = Box::new(TreeNode::new(val, origin_pos));
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut current = &mut self.root;
        loop {
            match current {
                Some(node) => {
                    if val < node.val {
                        if node.left.is_none() {
                            node.left = Some(new_node);
                            return;
                        } else {
                            current = &mut node.left;
                        }
                    } else {
                        if node.right.is_none() {
                            node.right = Some(new_node);
                            return;
                        } else {
                            current = &mut node.right;
                        }
                    }
                },
                None => return,
            }
        }
    }

    // search value return its original position in sequence(BST->pos_in_sequence)
    pub fn search(&self, val: i32) -> Option<i32> {
        if self.root.is_none() {
            return None;
        }
        let mut current = &self.root;
        loop{
            match current {
                Some(node) => {
                    if val == node.val {
                        return Some(node.origin_pos_in_sequence);
                    } else if val < node.val {
                        current = &node.left;
                    } else {
                        current = &node.right;
                    }
                },
                None => return None,
            }
        }
    }
}

/// 从序列构建一个二叉排序树
pub fn build_bst(sequence: &[i32]) -> BST {
    // 按理说从序列中构建BST应该先排序再插入 以保证树的平衡 但这里我们直接插入原始序列以符合题意
    let mut bst = BST::new();
    
    for (index, &value) in sequence.iter().enumerate() {
        bst.insert(value, index as i32);
    }

    bst
}

/// 对一个序列查找多个目标元素，返回其索引 bst
pub fn search_n_bst(sequence: &[i32], test_case_for_search: &[i32]) -> Vec<usize> {
    println!("{}", "使用二叉排序树查找多个目标元素...".bright_black());

    let mut result = Vec::new();

    let bst = build_bst(sequence);

    let start = Instant::now();

    for &target in test_case_for_search {
        // 假设未找到时，我们添加一个特殊值（例如 usize::MAX）或直接忽略
        // 这里我们使用 unwrap_or(-1) 作为占位符，表示未找到则返回-1
        result.push(bst.search(target).unwrap_or(-1) as usize);
    }

    let duration = start.elapsed();
    println!("耗时: {:?}", duration);

    // 避免没有使用变量而警告
    let _ = (sequence, test_case_for_search, &result);

    result
}