fn main() {
    let number = 13;
    if number < 5 {
        println!("number 小于 5");
    } else if number > 10 {
        println!("number 大于 10");
    } else {
        println!("number 大于 5 小于等于 10")
    }

    let number = if number > 10 { 1 } else { -1 };

    println!("number is {}", number);
}
