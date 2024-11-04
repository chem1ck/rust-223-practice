/// https://practice.course.rs/variables.html
#[test]
fn test31() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test399() {
    let x = 5;
    let y = 10;

    let real = x + y;
    let expected = 15;

    assert_eq!(real, expected);
}

#[test]
fn test1() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test2() {
    let x: i32 = 10;
    let y: i32 = 5;

    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

#[test]
fn test3() {
    let x = define_x();
    println!("{}, world", x);
}
fn define_x() -> &'static str {
    "hello"
}

#[test]
fn test4() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

#[test]
fn test5() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

#[test]
fn test6() {
    let x = 1;
    println!("X = {}", x);
}

#[test]
fn test7() {
    let (x, y) = (1, 2);
    let mut x = x;
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]
fn test8() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}