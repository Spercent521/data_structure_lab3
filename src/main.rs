/// 
/// 题目要求 找到最大的$n$个数字
/// 对正序序列采用顺序搜索、折半搜索、二叉排序树、平衡二叉排序树进行搜索；
/// 对逆序序列采用顺序搜索、二叉排序树、平衡二叉排序树进行搜索；
/// 对随机序列采用顺序搜索、二叉排序树、平衡二叉排序树进行搜索.
/// 
/// 找到最大的数字使用折半搜索不是很好 改成搜索随机的n个数字
/// ==============================================
/// 我的实现
/// 完成可复用的 1. 顺序搜索 2. 折半搜索 3. 二叉排序树 4. 平衡二叉排序树 搜索
/// 
/// prompt:
/// 我刚刚学习rust 对变量的使用还不是很熟悉
/// 我需要逆帮我重新fix line66 以内的变量使用 以符合规范 后续我想沿用你给我的规范 你可能还需要帮我fix linear.rs中的函数头
/// 
pub mod sort_method;

use colored::*;

/*
 * `i32` 和 `usize`的选择问题
 * =====================================================================
 * 好的，我明白了.我们来专门讨论 `i32` 和 `usize` 在您代码中的选择是否合理.
 * 
 * **结论：您当前对 `i32` 和 `usize` 的使用是完全合理且正确的.**
 * 
 * 下面是详细解释：
 * 
 * 1.  **`sequence: &[i32]` 和 `test_case_for_search: &[i32]`**:
 *     *   **为什么用 `i32`？** `i32` 代表一个32位有符号整数.您用它来存储序列中的 **数值**.这是非常合适的选择，因为它：
 *         *   能表示正数、负数和零，是通用的整数类型.
 *         *   虽然您当前的 `make_random` 函数只生成非负数，但将函数签名定义为接受 `i32` 使得您的搜索函数更加通用和健壮，未来也可以处理包含负数的序列.
 *     *   **为什么不用 `usize`？** `usize` 的语义是用于表示内存大小和集合索引.它的具体大小（32位或64位）取决于目标机器的架构.将它用于存储通用的数值，虽然在数值都是非负的情况下可行，但会造成语义上的混淆.**`usize`应该保留给它本来的用途：索引和大小.**
 * 
 * 2.  **`-> Vec<usize>`**:
 *     *   **为什么用 `usize`？** 函数的目标是返回找到的元素的 **索引**.在 Rust 中，任何集合（数组、切片、Vec）的索引都必须是 `usize` 类型.因此，返回一个 `Vec<usize>` 是完全正确的.
 * 
 * **总结一下**：
 * 
 * *   用 `i32` (或 `i64`, `u32` 等) 来表示你的 **数据**.
 * *   用 `usize` 来表示 **索引** 或 **大小**.
 * 
 * 您当前的代码完美地遵循了这一原则.因此，您不需要对 `search_n_linear` 函数签名中的 `i32` 或 `usize` 进行任何修改.这个选择是符合 Rust 编程惯例的.
 */

/// 使用简单 LCG 根据 seed 生成随机序列（确定性，便于测试）
fn make_random(n: usize, seed: u32) -> Vec<i32> {
    // 生成包含 n 个伪随机整数的向量，数值范围在 [0, maxv-1] 之间.
    let maxv = (n as u32).saturating_mul(10).max(1);            //将 n 乘以 10 作为随机数的上限
                                                                     //使用 saturating_mul 防止溢出
                                                                     //使用 max(1) 确保至少为 1
    let mut x = seed.wrapping_add(0x1234_5678);
    (0..n).map(|_| {
        x = x.wrapping_mul(1664525).wrapping_add(1013904223); // LCG 参数
        (x % maxv) as i32
    }).collect()
}

/// 将结果按每行20个数字的格式打印出来 测试用例不应该超过6位数 为了对齐输出结果
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

/// 从给定的`sequence`中，选取 n_top_num 个数字作为搜索目标
#[allow(dead_code)]
fn generate_search_targets(sequence: &[i32], n_top_num: usize) -> Vec<i32> {
    // 使用`&[i32]`作为输入类型以避免不必要的克隆
    // 这里我们均匀选取 n_top_num 个数字 以保证覆盖整个范围
    let len = sequence.len();
    let step = len / n_top_num.max(1); // 防止除以零

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
        
        let n_top_num = (test_case / 100).max(1);                       // 此处一定要使用小写字母开头的变量名，否则不会被解析为变量

        // 1. #################################################################
        println!("{}", "[INFO] generating test_case_sequence numbers...".cyan());

        // 这里的思路应该是 先用随机生成 再分别对其排序 或者先生成一个asc 再对其做shuffle打乱
        // 这两种算法各有优缺点 最后ai选择了前一种 可能是先随机的数据更强

        // shuffle方法
        // vec.shuffle(&mut thread_rng());

        // 总之要保持这三组数字一致 这样只用生成一组测试数据 保证公平
        let rand = make_random(test_case, 0xDEADBEEF);
        // // 这一段为什么有的用mut 我后面应该都不会改变他们
        // let mut asc = rand.clone();
        // asc.sort_unstable();
        // let mut desc = asc.clone();
        // desc.reverse();
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
        // // let test_case_for_search_asc:Vec<i32> = generate_search_targets(&asc, n_top_num);
        // // let test_case_for_search_desc:Vec<i32> = generate_search_targets(&desc, n_top_num);
        // // let test_case_for_search_rand:Vec<i32> = generate_search_targets(&rand, n_top_num);

        // 或者我们按照要求 将test_case_for_search:Vec<i32>选为最大的$n$个数字
        let test_case_for_search:Vec<i32> = asc[asc.len()-n_top_num..].to_vec();

        println!("{}", format!("[INFO] have generated test_case_for_search with {} numbers", n_top_num).cyan());

        // 3. #################################################################

        // part::搜索::正向排序
        println!("{}", "= 正向序列 =".yellow());
        let result_of_linear: Vec<usize> =
            sort_method::linear::search_n_linear(&asc, &test_case_for_search);
        show_top_n(&asc, &result_of_linear);
        let result_of_binary:Vec<usize> = sort_method::binary::search_n_binary(&asc, &test_case_for_search);
        show_top_n(&asc, &result_of_binary);
        // let result_of_bst:Vec<i32> = sort_method::bst::find_top_n_bst(&asc, n_top);
        // show_top_n(result_of_bst);
        // let result_of_avl:Vec<i32> = sort_method::avl::find_top_n_avl(&asc, n_top);
        // show_top_n(result_of_avl);
        

        // part::搜索::逆向排序
        println!("{}", "= 逆向序列 =".yellow());
        let result_of_linear:Vec<usize> = 
            sort_method::linear::search_n_linear(&desc, &test_case_for_search);
        show_top_n(&desc, &result_of_linear);
        let result_of_binary:Vec<usize> = sort_method::bst::search_n_bst(&desc, &test_case_for_search);
        show_top_n(&desc, &result_of_binary);
        // let result_of_avl:Vec<i32> = sort_method::avl::find_top_n_avl(&desc, n_top); 
        // show_top_n(result_of_avl);
        
        // part::搜索::随机排序
        println!("{}", "= 随机序列 =".yellow());
        let result_of_linear:Vec<usize> = 
            sort_method::linear::search_n_linear(&rand, &test_case_for_search);
        show_top_n(&rand, &result_of_linear);
        let result_of_binary:Vec<usize> = sort_method::bst::search_n_bst(&rand, &test_case_for_search);
        show_top_n(&rand, &result_of_binary);
        // let result_of_avl:Vec<i32> = sort_method::avl::find_top_n_avl(&rand, n_top);
        // show_top_n(result_of_avl);

        println!("{}", format!("==== Test case {} done. ====", test_case).magenta().bold());
    }
}
