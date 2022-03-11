pub fn initialise_variable() {
    let x: i32 = 2;
    assert_eq!(x, 2);
    println!("initialise_variable: initialise variable!")
}

pub fn mutable_variable() {
    let mut x: i32 = 2;
    x += 1;
    assert_eq!(x, 3);
    println!("mutable_variable: updated value of mutable variable!")
}

pub fn variable_scope() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("variable_scope: variable value of y: {}", y)
    }
    println!("variable_scope: variable value of x: {}", x)
}

pub fn shadowing() {
    let x: i32 = 10;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 10);
    let x = 42;
    println!("shadowing: variable value of x: {}", x);
}

pub fn shadowing_rebinding() {
    let mut x: i32 = 10;
    println!("shadowing_rebinding: variable value of x: {}", x);
    x = 7;
    let x = x;
    println!("shadowing_rebinding: variable value of x: {}", x);
    let y = 4;
    println!("shadowing_rebinding: variable value of y: {}", y);
    let y = "I can also be bound to text!";
    println!("shadowing_rebinding: variable value of y: {}", y);
}

pub fn destructuring() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("destructuring: success!")
}
