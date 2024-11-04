/// https://practice.course.rs/ownership/ownership.html
#[test]
fn test511() {
    fn main() {
        // Use as many approaches as you can to make it work
        let x = String::from("Hello world");
        let y = x.clone();
        println!("{}, {}",x, y);
    }
}
#[test]
fn test512() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

#[test]
fn test513() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s: Vec<char> = s.chars().collect();
    s
}

#[test]
fn test514() {
    let s = String::from("Hello World");

    let s2 = print_str(s);

    println!("{}", s2);
}

fn print_str(s: String) -> String {
    println!("{}",s);
    s
}

#[test]
fn test515() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

#[test]
fn test516() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

#[test]
/*fn main() {
    let x = Box::new(5);

    let ...      // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}*/

#[test]
fn test518() {
    let t = (String::from("hello"), String::from("world"));

    let _s = &t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t);
}

#[test]
fn test519() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = ("hello", "world");

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
#[test]
fn test521() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

#[test]
fn test522() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(*y, x);

    println!("Success!");
}

#[test]
fn test523() {
    let mut s = String::from("hello, ");

    borrow_object(s);

    println!("Success!");
}

fn borrow_object(s: String) {}

#[test]
fn test524() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

#[test]
fn test525() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success!");
}

#[test]
fn test526() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

#[test]
fn test527() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    println!("{}", r1);

    println!("Success!");
}

#[test]
fn test528() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

#[test]
fn test529() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}

#[test]
fn test5210() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    //let r2 = &mut s;
    //r2.push_str("!");

    println!("{}",r1);
}

//#[test]
/*fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}*/

