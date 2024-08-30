fn main() {
    let s = "hello";
    println!("{}", s);

    let mut s = String::from("hellwp");
    println!("{}", s);

    s.push_str(" world");
    println!("{}", s);

    let mut x = "heow";
    let y = x.clone();
    println!("{}, {}", x, y);
    x = "sss";
    println!("{}, {}", x, y);
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);

    // 函数变量的触发
    // let xzz = String::from("hello");
    // takes_ownership(xzz);
    // println!("{}", xzz);
    // let x = 5;
    // makes_copy(x);

    // fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s_len = calculate_length(&s);
    println!("{}", s_len);





    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &s; // 大问题
    println!("{}, {}, and {}", r1, r2, r3);
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn calculate_length(s: &String) -> usize {
    s.len()
}