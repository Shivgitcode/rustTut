fn main() {
    // let x = -5; //int
    // let y: u32 = 1000; //unsigned int
    // let z: f32 = 1000.001; //float
    // let is_male: bool = true;
    // // let ax = "adakdfjakdla"; // can change space at runtime
    // let greeting = String::from("hello world");

    // println!("x:{}, y:{}, z:{}, isMale:{}", x, y, z, is_male);
    // println!("{}", greeting)

    // let is_even = true;
    // if is_even {
    //     println!("The number is even");
    // } else {
    //     println!("The number is odd");
    // }

    // for i in 0..10 {
    //     println!("{}", i);
    // }
    let a: i32 = 10;
    let b: i32 = 20;
    let sum: i32 = do_sum(a, b);
    let name = String::from("Shivansh");
    println!("Hello {}", greet(name));
    println!("Sum of {} and {} is {}", a, b, sum);
}

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn greet(name: String) -> String {
    return name;
}
