#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(unused_variables)]

fn hail_step(n: i32) -> i32 {
    if n % 2 == 1 {
        (3 * n + 1) / 2
    } else {
        n / 2
    }
}

fn hail_seq(mut n: i32) {
    loop {
        print!("{} ", n);
        if n == 1 {
            break;
        }
        n = hail_step(n);
    }
}

/*
let j = 10;
let k = 19;
for i in j .. k {
    println!("for = {}", i);
}*/

fn hail_sieve() {
    let max_pass = 7;
    let max_cnt = 10;
    //let max_num = 60;
    println!("\nsmallest counter example");
    for pass in 1..max_pass + 1 {
        println!(">>> num_pass {}", pass);
        let mut cnt = 0;
        let mut n_orig = 1;
        while cnt < max_cnt {
            let mut n = n_orig;
            for p in 1..pass + 1 {
                n = hail_step(n);
                if n <= n_orig {
                    break;
                }
            }
            if n > n_orig {
                println!("{}", n_orig);
                cnt += 1;
            }
            n_orig += 1;
        }
        /*
        let mut pass_cnt = 0;
        let n_orig = 1;
        let mut n = n_orig;
        while pass_cnt < max_pass_cnt {
            let mut p = 0;
            while p < max_pass {
                n = hail_step(n);
                p += 1;
            }
            if n > n_orig {
                println!("hail = {} : ", n_orig);
                pass_cnt += 1;
            }
            n += 1;
        } */
    }
}

fn hail_basic() {
    println!("some basic hail tests");
    for n in 1..17 {
        print!("hail = {} : ", n);
        hail_seq(n);
        println!();
    }
}

pub fn main2() {
    // hail tests, eliminate smaller numbers
    println!();
    hail_basic();
    // try to find first counter example
    hail_sieve();
    // basic concepts chapter 3
    return;
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
