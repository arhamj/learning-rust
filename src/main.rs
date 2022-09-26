use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("Value of x is {}", x);

    let mut mutable_x = 5;
    println!("Value of mutable x is {}", mutable_x);
    mutable_x = 7;
    println!("New value of mutable x is {}", mutable_x);

    const MAX: i32 = 100;
    println!("Const MAX has value {}", MAX);

    // Scalar types
    // int
    let i = 5;
    // float
    let f = 2.0;
    // bool
    let b = true;
    // char
    let c = 'a';
    println!("Integer {}, Float {}, Bool {}, Char {}", i, f, b, c);

    // Compound types
    // tuple
    let t = (i, f, b, c);
    // array
    let a = [1, 2, 3, 4];
    println!("Tuple is {:?}", t);
    println!("Array is {:?}", a);

    print_sum(10, 20);
    println!("{}", sum(10, 20));

    print_odd_or_even(10);
    print_odd_or_even(11);
    print_odd_or_even(0);

    loop_and_sum_list(a);

    let str = "Hello, World!";
    print_string(str);

    let mut str2 = String::from("Hello, World");
    println!("String pre mutation: {}", str2);
    change_string(&mut str2);
    println!("String post mutation: {}", str2);

    string_slice();

    struct_demo();

    struct_with_methods();

    let color_value = enum_match_demo(Colors::Black);
    println!("{}", color_value);

    vectors();

    maps()
}

fn print_sum(a: i32, b: i32) {
    println!("{}", a + b);
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn print_odd_or_even(num: i32) {
    if num != 0 && num % 2 == 0 {
        println!("Number {} is even", num)
    } else if num % 2 != 0 {
        println!("Number {} is odd", num)
    } else {
        println!("Zero is probably even")
    }
}

fn loop_and_sum_list(a: [i32; 4]) {
    let mut i = 0;
    let mut sum: i32 = 0;
    let result = loop {
        sum += a[i];
        i += 1;
        if i > 3 {
            break sum;
        }
    };
    println!("Sum using loop syntax is {}", result);

    i = 0;
    sum = 0;
    while i < 4 {
        sum += a[i];
        i += 1;
    }
    println!("Sum using while syntax is {}", sum);

    sum = 0;
    for elem in a.iter() {
        sum += elem;
    }
    println!("Sum using iterator on list is {}", sum)
}

fn print_string(s: &str) {
    println!("{}", s)
}

fn change_string(s: &mut String) {
    s.push_str(", Mutation")
}

fn string_slice() {
    let s = String::from("arham");
    let sl1 = &s[0..2];
    let sl2 = &s[2..4];
    println!("Slice 1: {}, Slice 2: {}", sl1, sl2);
}

struct User {
    first_name: String,
    last_name: String,
    active: bool,
}

fn struct_demo() {
    let user = User {
        first_name: String::from("Arham"),
        last_name: String::from("Jain"),
        active: true,
    };
    println!(
        "{} {} is {}",
        user.first_name,
        user.last_name,
        if user.active { "active" } else { "inactive" }
    )
}

fn struct_with_methods() {
    struct Rectangle {
        length: u64,
        breadth: u64,
    }

    impl Rectangle {
        fn area(&self) -> u64 {
            return self.length * self.breadth;
        }
    }

    let rect = Rectangle {
        length: 10,
        breadth: 11,
    };
    println!(
        "Area of rectangle with length {} and breadth {} is {}",
        rect.length,
        rect.breadth,
        rect.area()
    )
}

enum Colors {
    Black,
}

fn enum_match_demo(color: Colors) -> i32 {
    return match color {
        Colors::Black => 1,
    };
}

fn vectors() {
    let v = vec![1, 2, 3];
    println!("{:?}", v)
}

fn maps() {
    let mut scores: HashMap<String, i8> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Printing values in the map");
    for iter in scores.iter() {
        println!("\tkey => {}, value => {}", iter.0, iter.1)
    }
}
