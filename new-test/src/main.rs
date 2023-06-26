/*
fn main() {
    // let s ="HELLO";
    let mut s = String::from("Hello");
    s.push_str(",World");
    println!("here you are --> {s:?}")
}
*/


/*
fn main() {
    // let x = 5;
    // let y = x;
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("S1: {}",s1);
    println!("S2: {}",s2);
}

 */


/*
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
// s's value moves into the function...
// ... and so is no longer valid here
// x comes into scope
// x would move into the function,
// but i32 is Copy, so it's okay to
// still use x afterward
}

// Here, x goes out of scope, then s. But because s's value was moved,
// nothing special happens.
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
 */


// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


/*
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

 */


/*
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == u8::try_from('b').unwrap() {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally
    // invalid!
}

 */


fn main() {
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    println!("s: {}, slice: {}", s, slice);

    let slice = &s[..];
    println!("s: {}, slice: {}", s, slice);

}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] }