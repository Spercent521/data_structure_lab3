use std::time::Instant;
use colored::Colorize;

/// 对一个序列查找多个目标元素，返回其索引 bst
pub fn search_n_bst(sequence: &[i32], test_case_for_search: &[i32]) -> Vec<usize> {
    println!("{}", "使用二叉排序树查找多个目标元素...".bright_black());

    let start = Instant::now();

    let result = Vec::new();

    // TODO

    let duration = start.elapsed();
    println!("耗时: {:?}", duration);

    // 避免没有使用变量而警告
    let _ = (sequence, test_case_for_search, &result);

    result
}