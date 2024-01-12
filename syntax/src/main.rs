// constants

// fn main() {
//     const NUMBER: i32 = 20;
//     const FLOATING_NUMBER: f32 = 20.0;
//     println!("{}", NUMBER);

//     println!("{}", FLOATING_NUMBER);
//     data_type();
// }

//! data types
// fn data_type() {
//     let number: i32 = -120;
//     let number1: u32 = 120;
//     let number2 = 120;

//     println!("{}", number);
//     println!("{}", number1);
//     println!("{}", number2);
// }

// fn main(){
//     let num1: u32 = 120;
//     let num2 = 1230;
//     println!("{}",num1);
//     println!("{}",num2);

// }

// fn main() {
//     let character1 = 'e';
//     let character2: char = '_';
//     let character3 = 'R';
//     let character4 = '😃';

//     println!("{}", character1);
//     println!("{}", character2);
//     println!("{}", character3);
//     println!("{}", character4);
// }

// fn main(){
//     let str:char = 's';
//     println!("{}",str)
// }

// !enums;

// #[derive(Debug)]

// enum Weekdays {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday,
// }

// fn main() {
//     let kun1 = Weekdays::Monday;
//     let kun2 = Weekdays::Tuesday;

//     println!("{:?}", kun1);
//     println!("{:?}", kun2);
// }

// enum Status {
//     ACTIVE,
//     INACTIVE,
// }

// fn main() {
//     let active = Status::ACTIVE;
//     let inactive = Status::INACTIVE;
//     println!("{:?}", active);
//     println!("{:?}", inactive);
// }

// ! for loops

// fn main() {
//     let array = [1, 2, 3, 4, 5];

//     for item in array {
//         println!("{}", item);
//     }

//     // Inclusive range
//     for i in 0..=5 {
//         println!("{}", i);
//     }
//     // Output: 0 1 2 3 4 5

//     // Exclusive range
//     for i in 0..5 {
//         println!("{}", i);
//     }
//     // Output: 0 1 2 3 4

//     //
// }

// fn main() {
//     let numbers: &[i64] = &vec![1, 2, 3, 4, 5];
//
//     for (i, item) in numbers.iter().enumerate() {
//         println!("value: - {} - Index: {}", item, i);
//     }
// }


// fn main() {
//     // let nums:&[i64] = &vec![1,2,3,4,5,6,7,8,9,10];
//     let nums = &vec![1,2,3,4,5,6,7,8,9,10];
//
//     for (index, item ) in nums.iter().enumerate() {
//         println!(">>> index: {}, item: {}",index, item);
//         println!("with index: {}",nums[index]);
//     }
// }

// ! FUNCTION

// fn main() {

// let emp=person_info("Bakhrom".to_string(),19);
// println!("res {:?}",emp);

// calculator("".to_string(), 12, 2)

// message_sender("Bakhrom".to_string(),"Hello dear, I love rust".to_string());

// hi_func()
// }
// fn person_info(name:String,age:i64)->(String,i64){
//
//     return (name.to_string(),age)
// }
// fn calculator(str: String, num1: i64, num2: i64) {
//     if str == "-" {
//         println!("Result: {}", num1 - num2)
//     } else if str == "*" {
//         println!("Result: {}", num1 * num2)
//     } else if str == "+" {
//         println!("Result: {}", num1 + num2)
//     } else if str == "/" {
//         println!("Result: {}", num1 / num2)
//     } else { println!("I don't understand your request, Please check!") }
// }
// fn message_sender(name: String, message: String){
//     println!("FROM: {}\n{}",name,message);
// }
//
// fn hi_func() {
//     println!("HI, are you ok?");
// }

// fn main() {
//     let employee: (u32, &str, bool, u32) = (1, "John", false, 5000);
//
//     let emp = get_employee(employee);
//     println!("{:?}", emp);
// }
// fn get_employee(emp: (u32, &str, bool, u32)) -> (u32, &str, bool, u32) {
//     println!("{:#?}", emp);
//     return emp;
// }

// !: ANONYMOUS FUNCTIONS
// fn main() {
//     let display = || {
//         println!("Anonymous Functions without arguments");
//     };
//     display();
// }

// fn main() {
//     let anon = |first: u32, second: u32| -> u32 {
//         println!("Anonymous Functions withs arguments");
//         let result = first + second;
//         return result;
//     };
//     println!("{}", anon(3, 1));
// }

// ! READ FROM CONSOLE
// use std::env;
//
// fn main() {
//     println!("Please enter your Name");
//
//     let args: Vec<String> = env::args().collect();
//
//     let name = &args[0];
//     let _filename = &args[0];
//
//     println!("Name is {}", name);
// }
//
// use std::io;
//
// fn main() {
//     println!("Enter your name:");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name)
//         .expect("Failed to read line");
//     println!("Hello, {}!", name);
// }


// ! LOOP
// there are three loops
// For loop
// while loop
// Loop

// fn main() {
// loop {
//     // Print string to the console
//     // this is infinitive loop without break
//     println!("Message");
//     break
// }


// }

// fn main() {
//     let mut number = 0;
//     loop {
//         number += 1;
//         if number == 3 {
//             println!("print continue statement");
//
//             continue;
//         }
//         println!("{}", number);
//         if number == 5 {
//             println! ("Matched and exit from the loop");
//             break; // Exit this loop
//         }
//     }
// }

// ! SWITCH

// fn main() {
// let day = 1;
// match day {
//     1 => println!("MONDAY"),
//     2 => println!("TUESDAY"),
//     3 => println!("WEDNESDAY"),
//     4 => println!("THURSDAY"),
//     5 => println!("FRIDAY"),
//     6 => println!("SATURDAY"),
//     7 => println!("SATURDAY"),
//     _ => println!("INVALID DAY"),
// }
//     let str = '+';
//     match str {
//         '+' => {
//             println!("RES: {}", 12 + 1)
//         },
//         '-' => {
//             println!("RES: {}", 12 - 1)
//         },
//         _ => {
//             println!("HEch brigade karma")
//         }
//     }
// }

//
// fn main() {
//     let str = String::from("welcome");
//
//     match str.as_str() {
//         "welcome" => {
//             println!("Matched");
//         }
//         _ => {
//             println!("Not matched");
//         }
//     }
// }

// ! SWITVH WITH ENUM

// enum WEEKEND {
//     SATURDAY,
//     SUNDAY,
// }
// fn main() {
//     let weekend = WEEKEND::SUNDAY;
//     match weekend {
//         WEEKEND::SUNDAY => println!("SUNDAY"),
//         WEEKEND::SATURDAY => println!("SATURDAY"),
//         _ => println!("NOT WEEKEND"),
//     }
// }

// ! SWITCH OR OR...

// fn main() {
//     let day = 3;
//     match day {
//         1 | 2 | 3 | 4 | 5 => println!("Weekday"),
//         6 | 7 => println!("Weekend"),
//         _ => println!("Invalid week"),
//     }
// }

// ! SWITCH WITH RANGE

// fn main() {
//     let day = 7;
//     match day {
//         1 ..= 5 => println!("Weekday"),
//         6 | 7 => println!("Weekend"),
//         _ => println!("Invalid week"),
//     }
// }


// ! TUPLE
// fn main() {
//     let employee: (u32, &str, bool) = (1, "John", false);
//     let employee1 = (1, "John", false);
//
//     println!("{:?}", employee); // normal print
//     println!("{:#?}", employee1); // pretty print
// }

// fn main() {
//     let employee: (u32, &str, bool) = (1, "John", false);
//
//     let emp = pretty_print(employee);
//     println!("{:?}", emp);
// }
//
// fn pretty_print(emp: (u32, &str, bool)) -> (u32, &str, bool) {
//     println!("{:#?}", emp);
//     return emp;
// }


// fn main() {
//     let employee: (u32, &str, bool) = (1, "John", false);
//
//     let (id, name, active) = employee;
//     println!("{}", id);
//     println!("{}", name);
//     println!("{}", active);
// }

// ! VARIABLES

// fn main() {
//     let name: &str = "John";
//     let NAME: &str ="Salom";
//
//
// }


// fn main() {
// muttable and unmmutable
//     // let  number = 2;
//     let mut number =2;
//     number = 20;
//     println!("{}", number)
// }

// shadowing
// fn main() {
//     let name = "John";
//     println!("{}", name);
//
//     let name=123;
//     println!("{}", name);
//
// }

// multiple variable declare
/*fn main() {
    let (first, second, third) = (false, 25, "John");
    println!("first: {} | second : {} | third = {}", first, second, third);
}
*/


/*
Rustc : rust compiler
cargo : Rust package management tool
rustup: toolchain manager tool for installing and uninstall rust environment
*/

// ! ARRAY
// fn main() {
// let mut arr:Vec<i64>=vec![];
// println!("{:?}",arr);
// arr.push(12);
// println!("{:?}",arr);

//   let mut my_array: &[i32] = &[];
//   println!("{:?}", my_array);
//
//   my_array = &[1, 2, 3, 4, 5];
//   println!("{:?}", my_array);
//
// }


// ! WHILE
// fn main() {
//   let mut number = 20;
//   let mut sum = 0;
//
//   while number >= 1 {
//     sum = sum + number;
//     number = number - 1;
//   }
//   println!("The Sum is {sum}");
// }


fn main() {
    let mut number = 20;
    let mut sum = 0;
    'outer: while {
        sum = sum + number;
        number = number - 1;
        if number == 0 {
            break 'outer;
        }
        true
    } {}
    println!("The Sum is {}", sum);
}
