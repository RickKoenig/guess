pub fn main2() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("index 1 = {}", tup.1);

    println!("calling five = {}", five());

    let c = 'z';
    let z = 'Ƶ';
    println!("c1 = {}, c2 = {}", c, z);

    let a = [1., 2., 3., 4., 500., 44.4];
    let first: f32 = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);

    let q = -4;
    let h: i8 = q * q;
    println!("h = {}", h);

    let n1 = 44.4;
    let n2 = n1;
    println!("n1 = {}, n2 = {}", n1, n2);

    let s1 = String::from("hi");
    //let s2 = s1;
    let s2 = s1.clone();
    //println!("s2 = {}", s2);
    let s3 = show_str(s2);
    println!("s1 = {}, s2 = {}", s1, s3);
    //let (s3, len) = calculate_length(s2);
    let len = calc_length(&s3);
    println!("length of {} = {}", s3, len);

    let mut s4 = String::from("hey ho!");
    change(&mut s4);
}

fn calc_length(s: &String) -> usize {
    //s = &String::from("hiho");
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn show_str(s: String) -> String {
    println!("s = {}", s);
    s
}

fn five() -> f32 {
    5.25
}
