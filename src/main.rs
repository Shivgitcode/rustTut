struct User {
    active: bool,
    username: String,
    email: String,
    age: u32,
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

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
    // let a: i32 = 10;
    // let b: i32 = 20;
    // let sum: i32 = do_sum(a, b);
    // let name = String::from("Shivansh");
    // println!("Hello {}", greet(name));
    // println!("Sum of {} and {} is {}", a, b, sum);
    // let mut x: String = String::from("hi there");
    // x.push_str("asd");
    // println!("{}", x);
    let s1 = String::from("Hello");

    takes_ownership(s1.clone());

    println!("{}", s1);

    let name = String::from("Alice");
    let _user = User {
        username: name,
        age: 30,
        active: true,
        email: String::from("shivneeraj2004@gmail.com"),
    };

    let square = Rect {
        width: 30,
        height: 40,
    };

    println!("{} is {} years old", _user.username, _user.age);

    print!("area of square is {}", square.area())

    // let s2 = s1;
    // println!("{}", s1);
    // println!("{}", s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
// fn do_sum(a: i32, b: i32) -> i32 {
//     return a + b;
// }

// fn greet(name: String) -> String {
//     return name;
// }
