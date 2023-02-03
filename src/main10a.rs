#![allow(unused_variables)]

pub fn main10a() {
    {
        let r;
        {
            let x = 5;
            r = x; // copy
                   //r = &x; // reference
        }
        println!("r = {}", r);
    }
    {
        let a = 33;
        let a1: &i32 = &a;
        //let a2: &'a i32 = &a;
        let a3 = &a;
    }
    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("abcd");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("longest string = {}", result);
    }
}
