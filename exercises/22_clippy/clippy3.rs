// 这里还有一些更容易的Clippy修复示例，这样你就能看到它的用处了 📎
// TODO: 修复此所有Clippy lint(检查提示)。

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // 假设你不知道 `my_option` 的值。
    // 如果是 `Some` ，那么我们就打印它的值。
    if my_option.is_none() {
        println!("{}", my_option.is_none());
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;

    // 1. 创建一个临时变量来存储 value_a 的原始值 (45)
    let temp = value_a;

    // 2. 将 value_b 的值 (66) 赋给 value_a
    value_a = value_b;

    // 3. 将临时变量中存储的值 (45) 赋给 value_b
    value_b = temp;

// 结果: value_a = 66, value_b = 45
    println!("value a: {value_a}; value b: {value_b}");
}
