struct User {
    username: String,
    password: String,
    active: bool,
    sign_in_count: i32,
}

#[derive(Debug)]
struct Dimensions {
    width: i32,
    height: i32,
}

impl Dimensions {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        username: String::from("shivansh"),
        password: String::from("SHIV@ansh10"),
        active: true,
        sign_in_count: 2,
    };

    let mut user2 = User {
        username: String::from("shivansh"),
        password: String::from("s26082004"),
        ..user1
    };

    user2.username = String::from("Ronalod");

    let user3 = create_user(String::from("abhiraj"), String::from("12345"));

    println!(
        "{} ,{}, {}, {}",
        user3.username, user3.password, user3.active, user3.sign_in_count
    );
    let white = Color(0, 0, 0);
    let black = Color(255, 255, 255);
    println!("{}", white.0);
    println!("{}", black.1);

    let rectangle = Dimensions {
        width: 20,
        height: 30,
    };

    println!("{}", rectangle.area());
    // println!("{}", area(&rectangle))
    println!("{:?}", rectangle);
    let squ = Dimensions::square(3);
    println!("{}", squ.area())
}

fn area(rect: &Dimensions) -> i32 {
    rect.height * rect.width
}

fn create_user(username: String, password: String) -> User {
    User {
        username: username,
        password: password,
        active: true,
        sign_in_count: 3,
    }
}
