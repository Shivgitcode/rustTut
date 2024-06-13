fn main() {
    let mut str1 = String::from("hello world");
    // let str2 = str1;
    // let str2 = takes_ownership(str1);

    let str3 = &mut str1;

    // str3.push_str(" foor");
    change_string(str3);
    // let str4 = &mut str1;
    // let str5 = &mut str1;

    // let num1: u32 = 5;
    // let num2 = num1;
    // stack_variable(num1);

    println!("this is str1 {}", str1);
    str1.clear();
    // println!("this is str3 {}", str3);
    // println!("this is str4 {}", str4);
    // println!("this is str5 {}", str5);
    // println!("this is num1 {}", num1);
    // println!("this is num2 {}", num2);
    let str6 = &str1[..3];
    println!("this is str6 {}", str6)
}

// fn takes_ownership(str: String) -> String {
//     // println!("this is str1 {}", str);
//     str
// }

fn change_string(str: &mut String) {
    str.push_str(" how are you");
}

// fn stack_variable(num: u32) {
//     println!("this is num1 {}", num);
// }
