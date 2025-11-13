use std::time::Instant;

/// 对一个序列查找多个目标元素，返回其索引 binary
pub fn search_n_binary(sequence: &[i32], test_case_for_search: &[i32]) -> Vec<usize> {
    let start = Instant::now();

    let result = Vec::new();

    // TODO

    let duration = start.elapsed();
    println!("耗时: {:?}", duration);

    // 避免没有使用变量而警告
    let _ = (sequence, test_case_for_search, &result);

    result
}