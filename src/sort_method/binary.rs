use std::time::Instant;
use colored::Colorize;

pub fn check_increase(sequence: &[i32]) -> bool {
    for i in 1..sequence.len() {
        if sequence[i] < sequence[i - 1] {
            return false;
        }
    }
    true
}

/// 对一个序列查找多个目标元素，返回其索引 binary
pub fn search_n_binary(sequence: &[i32], test_case_for_search: &[i32]) -> Vec<usize> {
    println!("{}", "使用折半搜索(binary search) 查找多个目标元素...".bright_black());

    // 跳过非递增序列
    if !check_increase(sequence) {
        println!("序列不是递增的，无法使用二分查找.");
        return Vec::new();
    }else{
        println!("序列是递增的，继续使用二分查找.");
    }

    let start = Instant::now();

    let mut result = Vec::new();

    for &target in test_case_for_search {
        // 在sequence中进行二分查找
        let mut left = 0;
        let mut right = sequence.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if sequence[mid] == target {
                result.push(mid);
                break;
            } else if sequence[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
    }

    let duration = start.elapsed();
    println!("耗时: {:?}", duration);

    result
}