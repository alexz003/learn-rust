use std::io;

fn main() {
    // variables_mutability_shadowing();
    // overflow();
    // data_types();
    parameter(5);
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