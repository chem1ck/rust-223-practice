/// https://practice.course.rs/basic-types/numbers.html
#[test]
fn test1() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // Type of z ?

    println!("Success!");
}

#[test]
fn test2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
fn test3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

#[test]
fn test5() {
    let v1 = 251_u8 + 4;
    let v2 = u8::checked_add(251, 4).unwrap();
    println!("{},{}",v1,v2);
}

#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

#[test]
fn test7() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test8() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    // Modify the assertion to check for the correct sum
    assert!(sum == -5); // Change from -3 to -5

    // Print the ASCII values for 'a' and 'z'
    println!("{} - {}", 'a' as u32, 'z' as u32); // Output: 97 - 122
}

#[test]
use std::ops::{Range, RangeInclusive};
fn test9() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

#[test]
fn test10() {
    // Сложение целых чисел
    assert!(1u32 + 2 == 3); // 1 + 2 = 3

    // Вычитание целых чисел
    assert!(1i32 - 2 == -1); // 1 - 2 = -1
    assert!(1i8 - 2 == -1); // Исправлено: приведение типа для корректного вычитания

    assert!(3 * 50 == 150); // 3 * 50 = 150

    // Исправление: указываем тип явно для деления
    let result = 9.6f64 / 3.2f64;
    assert!((result - 3.0f64).abs() < 1e-10); // Сравнение с точностью

    assert!(24 % 5 == 4); // Остаток от деления 24 на 5 равен 4

    // Логические операции с коротким замыканием
    assert!(true && false == false); // true AND false дает false
    assert!(true || false == true); // true OR false дает true
    assert!(!true == false); // NOT true дает false

    // Побитовые операции
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

