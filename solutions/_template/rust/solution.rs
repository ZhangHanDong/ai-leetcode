/// 模板实现只演示解法文件的边界。
///
/// 复制模板后，应替换输入输出类型、函数名和测试，并把平台适配代码放在核心算法之外。
pub fn solve(input: &[i32]) -> Vec<i32> {
    input.to_vec()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn replace_with_problem_specific_case() {
        assert_eq!(solve(&[1, 2, 3]), vec![1, 2, 3]);
    }
}

