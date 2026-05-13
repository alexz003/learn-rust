use std::io;
use std::collections::HashMap;

fn main() {
    // variables_mutability_shadowing();
    // overflow();
    // data_types();
    // parameter(10);
    // control_flow();
    // ownership();
    // referencing_and_borrowing();
    // slices();
    // structs();
    // enums();
    // collections();
}

fn variables_mutability_shadowing() {
    let x = 5;
    
    println!("{x}");

    let x = x + 1;

    println!("{x}");

    {
        let x = x * 2;
        println!("inner scope: {x}")
    }

    println!("outer scope: {x}");

    let spaces = "     ";
    let spaces: usize = spaces.len();

    println!("spaces len: {spaces}");
}

fn overflow() {
    let mut guess: u8 = "42".parse().expect("Not a number!");

    guess = guess + 250;
}

fn data_types() {
    let x = 2.0;
    let y: f32 = 3.0;

    let z = 2;
    // let z = 2 + 2.0;

    let t = true;

    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;
    println!("{five_hundred}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn parameter(mut x: i32) {
    println!("{x}");

    x = 6;
    println!("{x}");

    let y = 5;
    let y = imm_param(y);
    println!("{y}");

    let tup1 = tup_return(10).1;
    println!("{tup1}");
}

fn imm_param(y: i32) -> i32 {
    let y = y+1;
    println!("{y}");

    return y;
}

fn tup_return(y: u8) -> (u8, u8) {
    (5, y)
}

fn control_flow() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of the number is {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("Result {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // println!("the value is: {a[index]}"); // Not Valid
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..=4).rev() {
        println!("number: {number}");
    }
}

fn ownership() {
    // let mut s: String = "hello"; <--- not value as "string" is type &str, not String
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str(", world");

    println!("s1 = {s1}, s2 = {s2}");
    
    // let s3 = String::from("hello");
    // let mut s4 = s3;
    // s4.push_str(", world");

    // println!("s3 = {s3}, s4 = {s4}"); <--- no longer valid as s3 has been shadowed by s4

    let s = String::from("hello");

    takes_ownership(s); // - move occurs because `s` has type `String`, which does not implement the `Copy` trait

    // println!("lost ownership: {s}"); <-- s was borrowed by takes_ownership, which called drop on s as String did not implement `Copy`. Compiler error

    let x = 5;

    makes_copy(x); // ints implement Copy so x is not borrowed by this call

    println!("copied: {x}"); 

    let s = String::from("hello");
    let s1 = takes_and_gives_back(s);

    println!("{s1}");

    let s = String::from("hello");

    let (s2, len) = calculate_length(s);
    println!("The length of '{s2}' is {len}.")
}

fn takes_ownership(some_str: String) {
    println!("{some_str}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn takes_and_gives_back(some_str: String) -> String {
    some_str
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn referencing_and_borrowing() {
    let mut s1 = String::from("hello");

    let len = calculate_length_ref(&mut s1); // <-- Passing a reference to a value does not drop the value

    println!("The length of {s1} is {len}"); // <-- s1 is still available because it was not dropped in calculate_length_ref
}

fn calculate_length_ref(str: &mut String) -> usize {
    let length = str.len();

    str.push_str(", world");

    str.len()
}

fn slices() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value "hello"

    //s.clear(); // <-- word is borrowing s, so s is no longer available to use
    
    println!("{word}");

    let s = "Hello, world!";

    let word = first_word_lit(&s[0..5]);

    println!("{word}");

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_lit(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
}

fn structs() {
    let mut user1 = build_user("someusername@email.com".to_string(), "someusername".to_string());

    user1.sign_in_count += 1;

    let username = &user1.username; // <-- create a reference to allow us to keep user1.username in place and not get shadowed
    let sign_in_count = &user1.sign_in_count;
    println!("{username} has signed in {sign_in_count} time(s).");

    let user2 = User {
        email: String::from("anotheremail@email.com"),
        ..user1 // <-- allows us to copy the rest of the properties from user1
    };

    let username = &user2.username;
    let sign_in_count = &user2.sign_in_count;
    let email = &user2.email;
    println!("{username} at {email} has signed in {sign_in_count} time(s).");

    let subject = AlwaysEqual;
}

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // insted of username: username, we can use the same variable name
        email, // same with email
        sign_in_count: 1
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddr {
    fn to_string(&self) -> String {
        match self {
            IpAddr::V4(a, b, c, d) => {
                format!("{}.{}.{}.{}", a, b, c, d).to_string()
            },
            IpAddr::V6(str) => {
                str.to_string()
            }
        }
    }
}

fn route(ip_kind: IpAddr) {

}

fn enums() {
    let ip = IpAddr::V4(127, 0, 0, 1);
    let ip2 = IpAddr::V6(String::from("::1"));

    println!("Address: {}", ip.to_string());
    println!("Address: {}", ip2.to_string());
}

fn collections() {
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];
    let val = v.get(2);

    println!("The third element is {}", &v[2]);
    match val {
        Some(val) => println!("The value is {}", val),
        None => println!("The value is unknown")
    };

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12)
    ];

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{s3}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} has {score} points.");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{field_name}: {field_value}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

