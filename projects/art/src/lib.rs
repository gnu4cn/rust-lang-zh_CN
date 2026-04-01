//! # 美术
//!
//! 用于建模美术概念的库。

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


pub mod kinds {
    /// 根据 RYB 颜色模型的原色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 根据 RYB 颜色模型的间色。
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 等量组合两种原色以创建
    /// 一种间色。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --跳过代码--
        SecondaryColor::Purple
    }
}
