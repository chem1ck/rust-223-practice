#[test]
use std::mem::size_of_val;
fn test1(){
        let c1 = 'a';
        assert_eq!(size_of_val(&c1),4);

        let c2 = '中';
        assert_eq!(size_of_val(&c2),4);

        println!("Success!");
}

#[test]
fn test2() {
        let c1 = '中';
        print_char(c1);
}

fn print_char(c : char) {
        println!("{}", c);
}

#[test]
fn test3() {
        let _f: bool = true;

        let t = false;
        if !t {
                println!("Success!");
        }
}

#[test]
fn test4() {
        let f = true;
        let t = true || false;
        assert_eq!(t, f);

        println!("Success!");
}

#[test]
fn test5() {
        let _v: () = ();

        let v = ();
        assert_eq!(v, implicitly_ret_unit());

        println!("Success!");
}

fn implicitly_ret_unit() {
        println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
        println!("I will return a ()");
}

#[test]
fn test6() {
        let unit: () = ();
        assert!(size_of_val(&unit) == 0);

        println!("Success!");
}