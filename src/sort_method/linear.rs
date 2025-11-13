use std::time::Instant;

/// 对一个序列查找多个目标元素，返回其索引
pub fn search_n_linear(sequence: &[i32], test_case_for_search: &[i32]) -> Vec<usize> {
    println!("使用顺序搜索(linear search) 查找多个目标元素...");
    let start = Instant::now();

    let mut result = Vec::new();

    for &target in test_case_for_search {
        for (index, &num) in sequence.iter().enumerate() {
            if num == target {
                result.push(index);
                break;
            }
        }
    }

    let duration = start.elapsed();
    println!("耗时: {:?}", duration);

    result
}