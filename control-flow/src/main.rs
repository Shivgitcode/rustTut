use std::io;
fn main() {
    let mut num1 = String::new();
    let condition: bool = true;
    let num2 = if condition {
        "eligible to drive"
    } else {
        "not eligible to drive"
    };
    io::stdin().read_line(&mut num1).expect("not valid");

    let num: i32 = num1.trim().parse().expect("invalid input");
    if num > 18 {
        println!("eligible to vote")
    } else if num == 18 {
        println!("you are eligible to vote");
    } else {
        println!("you are not eligible to vote")
    }
    println!("{}", num2);

    let ages = [10, 20, 30, 40, 50];
    for a in 1..10 {
        println!("number:{}", a);
    }
    for a in ages {
        println!("number : {}", a);
    }
}
