pub mod sort_method;

use colored::*;

fn make_random(n: usize, seed: u32) -> Vec<i32> {
    let maxv = (n as u32).saturating_mul(10).max(1);
    let mut x = seed.wrapping_add(0x1234_5678);
    (0..n).map(|_| {
        x = x.wrapping_mul(1664525).wrapping_add(1013904223);
        (x % maxv) as i32
    }).collect()
}

fn show_top_n(original_sequence: &[i32], pos: &[usize]) {
    // 使用`&[i32]`作为输入类型以避免不必要的克隆
    // 打印搜索结果 格式为 `"[{}]={}",pos[i],original_sequence[pos[i]]`
    for (i, &p) in pos.iter().enumerate() {
        print!("[{:>6}]={:>6}  ", p, original_sequence[p]);
        if (i + 1) % 5 == 0 {
            println!();
        }
    }

    // 如果最后一行没有打印完 也需要换行
    if pos.len() % 5 != 0 {
        println!();
    }
}

#[allow(dead_code)]
fn generate_search_targets(sequence: &[i32], n_top_num: usize) -> Vec<i32> {
    let len = sequence.len();
    let step = len / n_top_num.max(1);

    (0..n_top_num)
        .map(|i| sequence[i * step.min(len - 1)])
        .collect()
}

fn main() {
    let tests: &[usize] = &[
        500, 
        1000, 
        // 2000, 
        // 5000, 
        // 10000, 
        // 20000, 
        // 30000, 
        // 50000, 
        // 100000, 
        // 200000,
    ];

    for &test_case in tests {
        println!("{}", format!("==== Test case {} start. ====", test_case).magenta().bold());
        
        let n_top_num = (test_case / 100).max(1);

        // 1. #################################################################
        println!("{}", "[INFO] generating test_case_sequence numbers...".cyan());

        let rand = make_random(test_case, 0xDEADBEEF);
        let asc = {
            let mut temp = rand.clone();
            temp.sort_unstable();
            temp
        };
        let desc = {
            let mut temp = asc.clone();
            temp.reverse();
            temp
        };

        println!("{}", "[INFO] have generated test_case_sequence numbers".cyan());

        // 2. #################################################################
        println!("{}", format!("[INFO] generating test_case_for_search with {} numbers...", n_top_num).cyan());

        // // 从asc中均匀选取n_top_num个数字作为搜索目标
        // let test_case_for_search:Vec<i32> = generate_search_targets(&asc, n_top_num);

        // 或者我们按照要求 将test_case_for_search:Vec<i32>选为最大的$n$个数字
        let test_case_for_search:Vec<i32> = asc[asc.len()-n_top_num..].to_vec();

        println!("{}", format!("[INFO] have generated test_case_for_search with {} numbers", n_top_num).cyan());

        // 3. #################################################################

        // part::搜索::正向排序
        println!("{}", "= 正向序列 =".yellow());
        let result_of_linear: Vec<usize> =
            sort_method::linear::search_n_linear(&asc, &test_case_for_search);
        show_top_n(&asc, &result_of_linear);
        // TODO

        // part::搜索::逆向排序
        println!("{}", "= 逆向序列 =".yellow());
        let result_of_linear:Vec<usize> = 
            sort_method::linear::search_n_linear(&desc, &test_case_for_search);
        show_top_n(&desc, &result_of_linear);
        // TODO
        
        // part::搜索::随机排序
        println!("{}", "= 随机序列 =".yellow());
        let result_of_linear:Vec<usize> = 
            sort_method::linear::search_n_linear(&rand, &test_case_for_search);
        show_top_n(&rand, &result_of_linear);
        // TODO

        println!("{}", format!("==== Test case {} done. ====", test_case).magenta().bold());
    }
}
