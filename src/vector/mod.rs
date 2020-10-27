pub fn create() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("vector: {:#?}", v);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There us no third element."),
    }
}

pub fn read() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("vector: {:#?}", v);

    for i in &mut v {
        *i += 50;
    }
    println!("vector: {:#?}", v);
}
