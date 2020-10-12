fn main() {
    // 整型
    let decimal = 98_222;
    println!("decimal: {}", decimal);
    let hex = 0xff;
    println!("hex: {}", hex);
    let octal = 0o77;
    println!("octal: {}", octal);
    let binary = 0b0000_0010;
    println!("binary: {}", binary);
    let byte = b'A';
    println!("byte: {}", byte);
    // 字符类型
    let c = 'z';
    println!("c: {}", c);
    // 元组类型
    let tup = (500, 'A', true, 0.2);
    let (decimal, charactor, truthy, floating_number) = tup;
    println!("The value of decimal is: {}", decimal);
    println!("The value of charactor is: {}", charactor);
    println!("The value of truthy is: {}", truthy);
    println!("The value of floating_number is: {}", floating_number);
    println!("The value of floating_number is: {}", tup.3);

    // 数组
    let arr = [1, 2, 3];
    let arr = [3; 5];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
}
