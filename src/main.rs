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
// fn create_str() {
//     let mut s1 = String::from("Deep");
//     s1 = print_str(s1);
// 
//     println!("s1{}", s1);
// }

// fn print_str(s2: String) -> String {
//     println!("s2 {}", s2);
//     return s2; 
// }

// fn main() {
//     create_str();
// }

//borrowing
// fn create_str() {
//     let mut s1 = String::from("Deep");
//     print_str(&mut s1);

//     println!("s1{}", s1);
// }

// fn print_str(s2: &mut String) {
//     s2.push_str("jyoti");
//     println!("s2 {}", s2);
// }

// fn main() {
//     create_str();
// }


// collections 
//vector 
// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);

//     let ans = even_filter(&vec);
//     println!("{:?}", ans);
//     println!("{:?}", vec);
// }

// fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val);
//         }
//     }
//     return new_vec;
// }

// use std::collections::HashMap;

//HashMaps
// fn main() {
//     let mut users = HashMap::new();

//     users.insert(String::from("Deep"),22);
//     users.insert(String::from("jyoti"),23);

//     let first_user_age = users.get("Deep");
//     // println!("{:?}", first_user_age);

//     match first_user_age {
//         Some(age) => println!("age is {}", age),
//         None => println!("user not found")
//     }
// }

// fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
//     let mut hm = HashMap::new();
//     for (key, value) in vec {
//         hm.insert(key, value);
//     }
//     return hm;
// }

// fn main() {
//     let input_vec = vec![
//         (String::from("Deep"), 22),
//         (String::from("jyoit"), 23),
//         ];
    
//     let hm = group_values_by_keys(input_vec);
//     println!("{:?}", hm);
// }

//iterators
// fn main() {
//     // let mut v1 = vec![1,2,3];
//     let v1 = vec![1,2,3,4,5];

//     // let v1_iter = v1.into_iter();
//     // // let mut v1_iter = v1.iter_mut();

//     // // while let Some(val) = v1_iter.next() {
//     // //     *val = *val + 1;
//     // //     println!("{}", val);
//     // // }

//     // // for val in v1_iter {
//     // //     *val = *val + 1;
//     // //     println!("{}", val);
//     // // }

//     // for mut val in v1_iter {
//     //     val = val + 1;
//     //     println!("{}", val);
//     // }

//     // println!("{:?}", v1);

//     let v1_iter = v1.iter();

//     // let total_sum: i32 = v1_iter.sum();
//     // println!("{}", total_sum);

//     // let v2_iter = v1_iter.map( |x| x + 1);
//     // let v2_iter = v1_iter.filter( |x| *x % 2 == 0);

//     // let v2_iter = v1_iter.filter(|x| *x % 2 == 1).map(|x| x + 1);

//     // for i in v2_iter {
//     //     println!("{}", i);
//     // }

//     // let v2_vec: Vec<i32> = v2_iter.collect();

//     let v2_vec: Vec<i32> = v1_iter.filter(|x| *x % 2 == 1).map(|x|x + 1).collect();

//     println!("{:?}", v2_vec);

//     // for i in v1_iter {
//     //     println!("v1_iter {}", i);
//     // }

//     println!("{:?}", v1);
// }

// use std::collections::HashMap;

// fn get_hash(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
//     // let mut hm = HashMap::new();

//     // for (key, val) in vec {
//     //     hm.insert(key, val);
//     // }
//     // return  hm;

//     let iter = vec.into_iter();
//     let hm = iter.collect();
//     return  hm;
// }

// fn main () {
//     let mut vec1: Vec<(String, i32)>= Vec::new();

//     vec1.push((String::from("Deep"), 23));
//     vec1.push((String::from("Jyoti"), 24));

//     // let get_hm = get_hash(vec1);
//     let iter = vec1.into_iter();
//     let hm: HashMap<String, i32> = iter.collect();
//     println!("HashMap {:?}", hm);

//     // let get_hm_iter = get_hm.iter();
    
//     // for (key, val) in get_hm_iter {
//     //     println!(" key {}: value{}", key, val);
//     // }
//     // println!("HashMap iterator {:?}", get_hm_iter);

//     let iter2 = hm.into_iter();
//     println!("hashmap to iter {:?}", iter2);

//     let vec2: Vec<(String, i32)> = iter2.collect();
//     println!("iterator to vec {:?}", vec2);

// }

// Slices and Strings

fn main() {
    let name = String::from("Deep jyoti");
    let ans = first_word(name);
    println!("ans is {}", ans);
}

fn first_word(str: String) -> String {
    let mut ans = String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}