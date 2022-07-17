fn feedback(lhs: i32, rhs: i32) -> i32 {

    println!("lhs: {}, rhs: {}", lhs, rhs);

    match (lhs, rhs) {

        (13, 13) => println!("Well I'll be!"),
        (13..=15, 13..=15) => println!("Interesting..."),
        _ => println!("WRONG"),

    }

    return lhs + rhs;
}



fn main() {

    let a = feedback(13, 15);
    let c = feedback(15, 14);
    let b = feedback(13, 13);

    println!("{}", a + b + c);
}
