pub fn main3() {
    // ownership and borrowing chapter 4

    // copyable, numbers and such
    println!("\ncopyable");
    let q = -4;
    let h: i8 = q * q;
    println!("h = {}", h);

    // change a number by reference
    let n1 = 11;
    let mut n2 = n1;
    println!("n1 = {}, n2 = {}", n1, n2);
    change_num(&mut n2);
    println!("NEW n2 = {}", n2);

    // not copyable, Strings and such
    println!("\nNOT copyable");
    let s1 = String::from("hi");
    let s2 = s1.clone();
    let s3 = show_move_str(s2);
    println!("s1 = '{}', s2 = '{}'", s1, s3);
    let len = calc_length(&s3);
    println!("length of s3 '{}' = {}", s3, len);

    // change a string by reference
    let mut s4 = String::from("hey ho!");
    println!("length of s4 '{}' = {}", s4, s4.len());
    change_str(&mut s4);
    println!("length of NEW s4 '{}' = {}", s4, s4.len());

    // more than once mutable
    let s4o = String::from("test mut");
    let r1 = &s4o;
    let r2 = &s4o;
    //let r3 = &mut s4o;
    println!("{}, {}", r1, r2);
    println!("{}, {}, {}", r1, r2, s4o);
    last_half();
}

fn change_num(n: &mut i32) {
    let q: i32 = *n;
    *n = q * 3 + 1;
}

fn show_move_str(s: String) -> String {
    println!("s = {}", s);
    s
}
fn calc_length(s: &String) -> usize {
    s.len()
}

fn change_str(s: &mut String) {
    //s.push_str(" ADDED");
    *s = s.to_owned() + s;
    //s = s.to_owned() + "hi";
    //s.push_str(", world");
}

fn last_half() {
    let nothing = dangle();
    println!("nothing = {}", nothing);
    let str1 = "hi ho";
    let str2 = &String::from("THERE");
    let slice1 = first_word(&str1);
    let slice2 = first_word(&str2);
    println!("slice1 = {}, slice2 = {}", slice1, slice2);
    let hi = &str1[..2];
    let ho = &str1[3..];
    //str1.clear();
    println!("'{}', '{}'", hi, ho);
}

fn dangle() -> String {
    let s = String::from("hiho");
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (j, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..j];
        }
    }
    &s[..]
}
