use std::time::Instant;
use colored::Colorize;

struct AvlTreeNode {
    val: i32,
    origin_pos_in_sequence: i32,
    height: i32,
    left: Option<Box<AvlTreeNode>>,
    right: Option<Box<AvlTreeNode>>,
}

impl AvlTreeNode {
    fn new(val: i32 , origin_pos_in_sequence: i32) -> Self {
        AvlTreeNode { val, origin_pos_in_sequence, height: 1, left: None, right: None }
    }
}

pub struct AVL {
    root: Option<Box<AvlTreeNode>>,
}

impl AVL {
    pub fn new() -> Self {
        AVL { root: None }
    }

    pub fn insert(&mut self, val: i32, origin_pos: i32) {
        fn height(node: &Option<Box<AvlTreeNode>>) -> i32 {
            /* as_ref()处理 Option 而不获取其所有权的关键工具。它将 &Option<T> 转换为 Option<&T> */
            /* map_or()如果是None则返回空节点若非空则调用执行一个匿名函数 `|n| n.height` */
            /* 闭包 |n| n.height 是一个匿名函数，它的意思是返回节点的高度 */
            /* |n|：定义一个参数 n，代表从 Option 中解包出来的值 */
            node.as_ref().map_or(0, |n| n.height)
        }

        fn update_height(node: &mut Box<AvlTreeNode>) {
            node.height = 1 + std::cmp::max(height(&node.left), height(&node.right));
        }

        fn rotate_right(mut y: Box<AvlTreeNode>) -> Box<AvlTreeNode> {
            let mut x = y.left.take().expect("rotate_right requires left child");
            y.left = x.right.take();
            update_height(&mut y);
            x.right = Some(y);
            update_height(&mut x);
            x
        }

        fn rotate_left(mut x: Box<AvlTreeNode>) -> Box<AvlTreeNode> {
            let mut y = x.right.take().expect("rotate_left requires right child");
            x.right = y.left.take();
            update_height(&mut x);
            y.left = Some(x);
            update_height(&mut y);
            y
        }

        fn balance(mut node: Box<AvlTreeNode>) -> Box<AvlTreeNode> {
            update_height(&mut node);
            let bf = height(&node.left) - height(&node.right);
            if bf > 1 {
                if height(&node.left.as_ref().unwrap().left) < height(&node.left.as_ref().unwrap().right) {
                    let left = node.left.take().map(|l| rotate_left(l));
                    node.left = left;
                }
                return rotate_right(node);
            } else if bf < -1 {
                if height(&node.right.as_ref().unwrap().right) < height(&node.right.as_ref().unwrap().left) {
                    let right = node.right.take().map(|r| rotate_right(r));
                    node.right = right;
                }
                return rotate_left(node);
            }
            node
        }

        fn insert_rec(node: Option<Box<AvlTreeNode>>, val: i32, origin_pos: i32) -> Option<Box<AvlTreeNode>> {
            match node {
                None => Some(Box::new(AvlTreeNode::new(val, origin_pos))),
                Some(mut n) => {
                    if val < n.val {
                        n.left = insert_rec(n.left.take(), val, origin_pos);
                    } else {
                        n.right = insert_rec(n.right.take(), val, origin_pos);
                    }
                    Some(balance(n))
                }
            }
        }

        self.root = insert_rec(self.root.take(), val, origin_pos);  /* 注意这个take() 实现了拿走->(交给函数)修改->放回 */
    }

    pub fn search(&self, val: i32) -> Option<i32> {
        let mut cur = &self.root;
        while let Some(node) = cur {
            if val == node.val {
                return Some(node.origin_pos_in_sequence);
            } else if val < node.val {
                cur = &node.left;
            } else {
                cur = &node.right;
            }
        }
        None
    }
}

pub fn build_avl(sequence: &[i32]) -> AVL {
    let mut tree = AVL::new();
    for (i, &val) in sequence.iter().enumerate() {
        tree.insert(val, i as i32);
    }
    tree
}

/// 对一个序列查找多个目标元素，返回其索引 avl
pub fn search_n_avl(sequence: &[i32], test_case_for_search: &[i32]) -> Vec<usize> {
    println!("{}", "使用avl查找多个目标元素...".bright_black());

    let mut result = Vec::new();

    let avl = build_avl(sequence);

    let start = Instant::now();

    for &target in test_case_for_search {
        result.push(avl.search(target).unwrap_or(-1) as usize);
    }

    let duration = start.elapsed();
    println!("耗时: {:?}", duration);

    // 避免没有使用变量而警告
    let _ = (sequence, test_case_for_search, &result);

    result
}