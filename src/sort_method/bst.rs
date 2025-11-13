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
        todo!("实现将值和其原始位置插入二叉排序树的逻辑");
    }

    // search value return its original position in sequence(BST->pos_in_sequence)
    pub fn search(&self, val: i32) -> Option<i32> {
        todo!("实现在二叉排序树中查找值的逻辑，如果找不到则返回 None");
    }
}

/// 从序列构建一个二叉排序树
pub fn build_bst(sequence: &[i32]) -> BST {
    let mut bst = BST::new();
    
    todo!("从序列构建二叉排序树");

    bst
}

/// 对一个序列查找多个目标元素，返回其索引 bst
pub fn search_n_bst(sequence: &[i32], test_case_for_search: &[i32]) -> Vec<usize> {
    println!("{}", "使用二叉排序树查找多个目标元素...".bright_black());

    let start = Instant::now();

    let mut result = Vec::new();

    let bst = build_bst(sequence);

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