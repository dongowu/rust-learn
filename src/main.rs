fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn main() {
    let x = plus_or_minus(5);

    println!("The value of x is: {}", x);

    let mut jinye = String::from("吴今也");
    let my_string = String::from("小宝贝");

    for c in my_string.chars() {
        jinye.push(c)
    }

    println!("{}", jinye)
}
