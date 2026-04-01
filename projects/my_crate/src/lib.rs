//! # 我的代码箱
//!
//! `my_crate` 是个实用工具集，旨在让执行
//! 某些计算更加便捷。

/// 加一到给定的数字。
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq! (7, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
