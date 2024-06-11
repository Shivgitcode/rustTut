use std::io;

fn main() {
    let num1 = [1, 3, 6, 4];
    // let mut my_string = String::new();
    let mut x = String::new();
    let mut y = String::new();

    // io::stdin()
    //     .read_line(&mut my_string)
    //     .expect("cannot readline");

    // let num2: i32 = my_string.trim().parse().expect("not valid input");

    io::stdin().read_line(&mut x).expect("invalid");

    io::stdin().read_line(&mut y).expect("invalid");

    let x = x.trim().parse().expect("wrong input");
    let y = y.trim().parse().expect("wrong input");

    println!("num at index 0 is {}", num1[0]);
    // println!("print output {}", num2);go
    println!("sum of x and y are {}", sum(x, y));
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
