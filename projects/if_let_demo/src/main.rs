#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println! ("使用你喜欢的颜色，{color}，作为背景");
    } else if is_tuesday {
        println! ("周二是绿色的一天！");
    } else if let Ok(age) = age {
        if age > 30 {
            println! ("使用紫色作为背景色");
        } else {
            println! ("使用橙色作为背景色");
        }
    } else {
        println! ("使用蓝色作为背景色");
    }
}
