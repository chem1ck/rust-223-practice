#[test]
enum Direction {
    East,
    West,
    North,
    South,
}

fn test81() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}

#[test]

fn test82() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

#[test]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test83() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")
    }
}

#[test]
fn test84() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        if matches!(ab, 'a'..='z' | 'A'..='Z') {
            assert!(true); // Если это буква, то утверждение проходит
        }
    }

    println!("Success!");
}

#[test]
enum MyEnum {
    Foo,
    Bar
}

fn test85() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { // Fix the error by changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

#[test]

fn test86() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}

#[test]
enum Foo {
    Bar(u8)
}

fn test87() {
    let a = Foo::Bar(1);

    match a {
        Foo::Bar(i) => {
            println!("foobar holds the value: {}", i);
            println!("Success!");
        }
    }
}

#[test]
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn test88() {
    let a = Foo::Qux(10);

    match a {
        Foo::Bar => {
            println!("match foo::bar");
        },
        Foo::Baz => {
            println!("match foo::baz");
        },
        _ => {
            println!("match others");
        }
    }
}

#[test]
fn test89() {
    let age = Some(30);
    if let Some(inner_age) = age { // Create a new variable with the same name as previous `age`
        assert_eq!(inner_age, 30);
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}

#[test]