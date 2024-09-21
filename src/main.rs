// fn main() {
//     let ans = is_even(20);
//     println!("{}",ans);
// }

// fn is_even(num: i32) -> bool{
//     if num %2 == 0 {
//         return  true;
//     } else {
//         return false;
//     }
// }

// fibonacci series
// fn main() {
//     println!("{}", fib(3));
// }

// fn fib(num: u32) -> u32 {
//     let mut first = 0;
//     let mut second = 1;

//     if num == 0 {
//         return 0;
//     }

//     if num == 1 {
//         return 1;
//     }

//     for _ in 0..(num - 1) {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }
//     return second;
// }

// fn main() {
//     let name = String::from("Deep");
//     let length = get_str_len(name);
//     println!("the length of the string is {}", length);
// }

// fn get_str_len(str: String) -> usize {
//     str.chars().count()
// }

//Structs
// struct User {
//     first_name: String,
//     last_name: String,
//     age: i32,
// }

// fn main() {
//     let user = User {
//         first_name: String::from("Deepjyoti"),
//         last_name: String::from("Sarmah"),
//         age: 23,
//     };
//     println!("{}", user.first_name);
// }

// struct Rect {
//     width: i32,
//     height: i32,
// }

// impl Rect {
//    fn area(&self) -> i32 {
//     self.width * self.height
//    }

//    fn perimeter(&self) -> i32 {
//     2 * (self.width + self.height)
//    }

//    fn two_arg(&self, num: i32) -> i32 {
//     num
//    }

//    fn no_self() -> i32 {
//     1
//    }
// }

// fn main() {
//     let rect = Rect {
//         width: 10,
//         height: 20
//     };

//     println!("area is {}", rect.area());
//     println!("perimeter is {}", rect.perimeter());
//     println!("twoArg is {}", rect.two_arg(2));
//     // println!("noSelf is {}", rect.noSelf());
//     println!("noSelf is {}", Rect::no_self());
// }

//Enums
// enum Shape {
//     Rectangle(f64, f64), // width, height
//     Circle(f64), // radius
// }

// fn main() {
//     let my_shape = Shape::Rectangle(2.0,4.0);
//     println!("{}",print_area(my_shape));

//     let my_circle = Shape::Circle(2.0);
//     println!("{}",print_area(my_circle));
// }

// fn print_area(shape: Shape) -> f64 {
// println!("hi there");
//     let area = match shape {
//         Shape::Rectangle(a, b) => a * b,
//         Shape::Circle(r) => 3.14 * r * r,
//     };
//     return area;
// }
//

// enum CustomOption {
//     Some(i32),
//     None
// }

// Options -> Null value
// fn find_first_a(s: String) -> CustomOption{
//fn find_first_a(s: String) -> Option<i32> {
//     for (index, char) in s.chars().enumerate() {
//         if char == 'a' {
//             return CustomOption::Some(index as i32);
//            return Some(index as i32);
//            return index as i32;
//         }
//     }
//     return CustomOption::None;
//    return None;
//    return -1;
// }

// fn main() {
//     let index = find_first_a(String::from("preet"));

//     match index {
//         CustomOption::Some(value) => println!("index of {}", value),
//         CustomOption::None => println!("a not found"),
//     }
//    if index == -1 {
//        println!("a is not found");
//    } else {
//        println!("index is {}", index);
//    }
// }

//Result -> error handling
// use std::fs::read_to_string;

// fn main() {
//     let result = read_to_string("a.txt");

//     match result {
//         Ok(data) => println!("{}", data),
//         Err(err) => println!("Error reading file {}", err),
//     }
// }

// external package
// use chrono::{Local, Utc};

// fn main() {
//     let now = Utc::now();
//     println!("Current Date and time in UTC: {}", now);

//     let formatted = now.format("%Y-%m-%d %H:%M:%S");
//     println!("Formatted date and time: {}", formatted);

//     let local = Local::now();
//     println!("Current date and time in local: {}", local);
// }
//

// Move and ownership
fn main() {
    let s1 = String::from("Deep");

    print_str(s1);

//    println!("s1 {}", s1);
}

fn print_str(s2: String) {
    println!("s2 {}", s2);
}
