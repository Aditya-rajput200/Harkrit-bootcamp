// use std::fs::read_to_string;
// use std::{char, option, vec};
// use chrono::format::StrftimeItems;
// use chrono :: {Local,Utc};
// mod practice;
// use practice::{Summary, Aser};
// use std::collections::HashMap;

// struct User {
//     user_name: String,
//     last_name: String,
//     age: u32,
// }

// enum Shape {
//     rectangle(f64, f64),
//     square(f64),
//     circle(f64),
// }

// fn main() {

//      let user = ser {
//         name: String::from("Aditya"),
//         age: 20,
//     };
    
//     println!("{}", user.summarize());
//     let mut counter = 0;
//     loop {
       

//         println!("Hlw Aditya it is loop {} ", counter);
//           counter+=1;

//     };
//     let _ans = is_even(15);
//     let _fib_number = fib(6);
//     let name = String::from("Aditya");
//     let _len = len_of_string(name);
//     let _nameAditya = 50;
//     println!("This is acer{}",_nameAditya);
//       let _nameAditya = 5;
//      println!("{}",_nameAditya);

//     let _user = User {
//         user_name: String::from("Aditya"),
//         last_name: String::from("Singh"),
//         age: 30,
//     };

//     let _rect1 = Rect {
//         width: 15,
//         height: 20,
//     };

//     let _sqr = Shape::square(10.0);
//     let _rect = Shape::rectangle(15.5, 20.0);
//     let _cir = Shape::circle(5.0);

//     let _area = calculate_area(_sqr);

//     let index = search_char(String::from("Aditya"));
//     match index {
//         Some(vlaue) => println!("this is the index of:{}", vlaue),
//         None => println!("A does not founded"),
//     }

//     let result = read_file(String::from("a.txt"));
//       match result {

       
//             Ok(content) => println!("File content: {}", content),
//         Err(Error)=>  println!("Error: {} ",Error)
          
//       }

//       let time = Utc::now();
//       println!("The current time is :{} ", time);

//       let mut s1 = String::from("Aditya raj");
//       str_print(&s1);
//       println!("Here s1 prints or not{} ", s1);


//       // HashMaps
//      let mut users = HashMap::new();

//      users.insert(String::from("Aditya"), 23);
//           users.insert(String::from("Nishi"), 22);

     
//       let user_age  = users.get("Nishi");
//        match  user_age {

//         Some(age) => println!("The age of the user is {}", age),
//         None => println!("User does not exist in the hashMAPS")

           
//        }
       
//        //hashmap questions
//        let input_vec = vec![(String::from("Aditya"),20 ),(String::from("sweta"),22 )];
//        let hm = convert_string(input_vec);
      
//       println!("The hashmaps are as follow {:?} ", hm);

    
// }
//      // hashmaps
//      fn convert_string(vec:Vec<(String,i32) > ) ->HashMap<String,i32> {

//         let mut ad = HashMap::new();
//          for (key,value) in vec {
//             ad.insert(key, value);
//          }
//          return  ad;
//      }
      
//      fn str_print(s2:&String) {
//         println!("{} ", s2)
//      }   
// fn calculate_area(shape: Shape) -> f64 { 
//     let area = match shape {
//         Shape::circle(r) => 3.14 * r * r,
//         Shape::rectangle(a, b) => a * b,
//         Shape::square(c) => c * c,
//     };
//     return area;
// }

// fn is_even(num: i32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     } else {
//         return false;
//     }
// }
// // 0 1 1 2  3  5
// // solving the fibo serires
// fn fib(num: i64) -> u64 {
//     let mut first = 0;
//     let mut second = 1;

//     if num == 0 {
//         return first;
//     }

//     if num == 1 {
//         return second;
//     }

//     for _ in 1..num - 1 {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }
//     return second;
// }

// // checking the length of the string

// fn len_of_string(str: String) -> usize {
//     str.chars().count()
// }

// // using the structs

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         &self.width * &self.height
//     }
//     fn perimetre(&self) -> u32 {
//         2 * (&self.width + &self.height)
//     }
// }

// fn search_char(name: String) -> Option<i32> {
//     for (index, char) in name.chars().enumerate() {
//         if char == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn read_file(filePath: String) -> Result<String, String> {
//     let read = read_to_string(filePath);

//     match read {
//         Ok(data) => Ok(data),
//         Err(err) => Err(format!("Failed to read the file {}", err)),
//     }
// }





// pub trait  Summary {
//     fn summarize (&self) -> String;
// }

// struct  Restaurent {
//     name:String,
//     rating:f64
// }

// impl Summary for Restaurent {

//     fn summarize (&self) -> String {
//         return format!("The name of the restaurent is {}, and the rating of  the restaurent is {} ",self.name, self.rating )
//     }
    
// }

// fn main() {
//     let rest = Restaurent {
//         name : String::from("Litoz.CO"),
//         rating:4.5,
//     };
//     println!("{}", rest.summarize());
// }

use std::vec;


fn main () {
    let v = vec![1,2,3,4];
    println!("{:?} ",v);
}