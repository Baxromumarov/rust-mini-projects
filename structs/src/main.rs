// fn main() {
//     let names = (18, "Bax rom", "Tashkent");
//     println!("{}, {},{}", names.0, names.1, names.2)
// }
//
//
//
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }
//
// impl Person {
//     fn say_hello(&self) {
//         println!("Hello, my name is {}", self.name);
//     }
// }
//
// fn main() {
//     let peter = Person {
//         name: String::from("Peter"),
//         age: 27,
//     };
//     peter.say_hello();
// }



/*
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {  // No receiver, a static method
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {  // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {  // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {  // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prefix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}

 */

// fn main() {
//     let input = 'x';
//
//     match input {
//         'q'                   => println!("Quitting"),
//         'a' | 's' | 'w' | 'd' => println!("Moving around"),
//         '0'..='9'             => println!("Number input"),
//         _                     => println!("Something else"),
//     }
// }


/*
use serde::{Deserialize};
use serde_json;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}


fn main() {

    let json_str = r#"{"name": "John Doe", "age": 30, "is_student": true}"#;

    let person: Person = serde_json::from_str(json_str)
        .expect("Failed to parse JSON");

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Is Student: {}", person.is_student);
}
*/

/*
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

const PATH: &str ="/Users/macbookpro/rust-mini-projects/structs/languages.json";
struct Language{
    code: String ,
    native: String,
}

#[derive(Deserialize)]
struct Total{
    list: Vec<Language>
}
fn main() {

    let mut file = File::open(PATH).expect("Failed to open file");

    let mut json_str = String::new();
    file.read_to_string(&mut json_str).expect("Failed to read file");

    let person: Total = serde_json::from_str(&json_str).expect("Failed to parse JSON");

}
*/



/*
struct Triangle(i32, i32, i32);
struct Respnonse(bool);

fn main() {
    let request = Triangle(2, 13, 14);
    let result = is_correct_triangle(request);
    println!("{}", result.0);
}

fn is_correct_triangle(var: Triangle) -> Respnonse {
    let mut response: Respnonse = Respnonse(false);
    if var.0 + var.1 > var.2 && var.0 + var.2 > var.1 && var.2 + var.1 > var.0 {
        response.0 = true;
    }

    return response;
}

*/


/*
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:#?}");
}

 */


/*
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!(">>>>>HERE {:#?}", user2);
    // let response_user = build_user("test@email.com".parse().unwrap(), "baxromumarov1".parse().unwrap(),true,12);
    // println!("After: {:#?}", response_user);
}

fn build_user(email: String, username: String, active: bool, sign_in_count: u64) -> User {
    User {
        email,
        username,
        active,
        sign_in_count,
    }
}


 */


/*
fn main() {
    // let width = 30;
    // let height = 50;
    // println!(
    //     "The area of rectangle is {} square pixel",
    //     area(width, height),
    // );
    let rect1 = (30,50);
    println!(
        "The area of rectangle is {} square pixel",
        area(rect1),
    );
}

// fn area(width: i32, height: i32) -> i32 {
//     width * height
// }

fn area(dimension: (i32, i32)) -> i32 {
    dimension.0 * dimension.1
}
*/


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//     // let rect2 = Rectangle { width: 10, height: 10 };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }


// ! ENUMS

enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}








