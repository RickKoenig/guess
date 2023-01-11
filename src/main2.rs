pub fn main2() {
    // basic concepts chapter 3

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("index 1 = {}", tup.1);

    // expressions
    println!("calling five = {}", five());

    // unicode
    let c = 'z';
    let z = 'Ƶ';
    println!("c1 = {}, c2 = {}", c, z);

    // types
    let a = [1., 2., 3., 4., 500., 44.4];
    let first: f32 = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);
}

// expression return
fn five() -> f32 {
    5.25
}
