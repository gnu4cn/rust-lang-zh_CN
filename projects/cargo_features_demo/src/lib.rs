//! # Cargo 特性示例代码箱
//!
//! `cargo_features_demo` 是令到执行某些确切计算更便利
//! 的一些工具的集合。
//!

/// 将一加到所给数字。
/// # 示例（examples）
///
/// ```
/// let arg = 5;
/// let answer = cargo_features_demo::add_one(arg);
///
/// assert_eq! (6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests;
