/// https://practice.course.rs/compound-types/string.html
#[test]
fn test611() {
    let s = "hello, world";
    println!("Success!");
}
#[test]
fn test612() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

#[test]
fn test613() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

#[test]
fn test614() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

#[test]
fn test615() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test616() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

#[test]
fn test617() {
    let s = "hello, world";
    greetings1(s.to_string())
}

fn greetings1(s: String) {
    println!("{}", s)
}

#[test]
fn test618() {
    let s = "hello, world";
    let s1 = s.to_string();

    println!("Success!");
}

#[test]
fn tes619() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test6110() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r####"Hello, "##""####;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

#[test]
fn test6111() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}
#[test]
fn test6112() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
#[test]
fn test621() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}
#[test]
fn test622() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}

#[test]
fn test623() {
    // Fill the blank
    let list: [i32; 100] = [1;100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

#[test]
fn test624() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}

#[test]
fn test625() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}

#[test]
fn test626() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[0];

    println!("Success!");
}
#[test]
fn test631() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}

#[test]
fn test632() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

#[test]
fn test633() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

#[test]
fn test634() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

#[test]
fn test635() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}

#[test]
fn test636() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
    let letter = first_letter(&s).to_string();

    s.clear(); // error!

    println!("the first letter is: {}", letter);
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}
#[test]
fn test641() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

#[test]
fn test642() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}

#[test]
fn test643() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7);
    println!("too long tuple: {:?}", too_long_tuple);
}
#[test]
fn test644() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x,y,z) = tup;

    assert_eq!(x, 1);
    assert_eq!(z, "hello");
    assert_eq!(y, 6.4);

    println!("Success!");
}

#[test]
fn test645() {
    let (x, y, z);

    // Fill the blank
    (y,z,x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

#[test]
fn test646() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2,3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

#[test]
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn test651() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("abc"),
    };

    println!("Success!");
}

#[test]
/*struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: __) {   }*/

#[test]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test653() {
    let v = Color(0, 127, 255); // Создаем экземпляр структуры Color
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let Color(x, y, z) = p; // Правильный способ распаковки структуры Color
    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255); // Проверяем значение z
}

#[test]
struct Person1 {
    name: String,
    age: u8,
}
fn test654() {
    let age = 18;
    let mut p = Person1 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

#[test]
struct Person3 {
    name: String,
    age: u8,
}
fn test655() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        age,
        name,
    }
}

#[test]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn test656() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    }
}

#[test]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test657() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("rect1 is {:?}", rect1); // Print debug info to stdout
}

#[test]
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn test658() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = &f.name;

    // ONLY modify this line
    println!("{}, {}, {:?}",f.name, f.data, f);
}

#[test]
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn test661() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number1::One as i32, Number2::One as i32);

    println!("Success!");
}

#[test]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test662() {
    let msg1 = Message2::Move{x: 1, y: 2}; // Instantiating with x = 1, y = 2
    let msg2 = Message2::Write(String::from("hello, world!")); // Instantiating with "hello, world!"

    println!("Success!");
}

#[test]
enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test663() {
    let msg = Message1::Move{x: 1, y: 2};

    if let Message1::Move{x: a, y: b} = msg {
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

#[test]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test664() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in &msgs {
        show_message(msg)
    }
}

fn show_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Message: Quit"),
        Message::Move { x, y } => println!("Message: Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Message: Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Message: ChangeColor to RGB({}, {}, {})", r, g, b),
    }
}

#[test]
fn test665() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    }

    panic!("NEVER LET THIS RUN！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

//#[test]
/*use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> __ {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // After Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // Instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, __ tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.__())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}*/