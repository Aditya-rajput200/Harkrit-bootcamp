use std::fs::read_to_string;
use std::{char, option};
use chrono :: {Local,Utc};
struct User {
    user_name: String,
    last_name: String,
    age: u32,
}

enum Shape {
    rectangle(f64, f64),
    square(f64),
    circle(f64),
}

fn main() {
    let _ans = is_even(15);
    let _fib_number = fib(6);
    let name = String::from("Aditya");
    let _len = len_of_string(name);

    let _user = User {
        user_name: String::from("Aditya"),
        last_name: String::from("Singh"),
        age: 30,
    };

    let _rect1 = Rect {
        width: 15,
        height: 20,
    };

    let _sqr = Shape::square(10.0);
    let _rect = Shape::rectangle(15.5, 20.0);
    let _cir = Shape::circle(5.0);

    let _area = calculate_area(_sqr);

    let index = search_char(String::from("Aditya"));
    match index {
        Some(vlaue) => println!("this is the index of:{}", vlaue),
        None => println!("A does not founded"),
    }

    let result = read_file(String::from("a.txt"));
      match result {

       
            Ok(content) => println!("File content: {}", content),
        Err(Error)=>  println!("Error: {} ",Error)
          
      }

      let time = Utc::now();
      println!("The current time is :{} ", time)
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::circle(r) => 3.14 * r * r,
        Shape::rectangle(a, b) => a * b,
        Shape::square(c) => c * c,
    };
    return area;
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
// 0 1 1 2  3  5
// solving the fibo serires
fn fib(num: i64) -> u64 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 1..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

// checking the length of the string

fn len_of_string(str: String) -> usize {
    str.chars().count()
}

// using the structs

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
    fn perimetre(&self) -> u32 {
        2 * (&self.width + &self.height)
    }
}

fn search_char(name: String) -> Option<i32> {
    for (index, char) in name.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn read_file(filePath: String) -> Result<String, String> {
    let read = read_to_string(filePath);

    match read {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Failed to read the file {}", err)),
    }
}
