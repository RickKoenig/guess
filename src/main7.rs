#![allow(unused_variables)] //on by default
#![allow(unused_must_use)]
use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;

pub fn main7() {
    {
        //panic!("crash and burn");
        let v = vec![1, 2, 3];
        v[2];
        println!("hi");
    }
    {
        let fname1 = String::from("hiho.txt");
        let fname2 = String::from("hiho.txt");

        let f = File::open(fname1);
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(fname2) {
                    Ok(fc) => fc,
                    Err(e) => panic!("can't open the file ''{:?}''", e),
                },
                other_error => panic!("can't open the file ''{:?}''", other_error),
            },
        };
    }
    {
        let fname1 = String::from("hiho.txt");
        let fname2 = String::from("hiho.txt");

        let f = File::open(fname1).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(fname2).unwrap_or_else(|error| {
                    panic!("can't create the file ''{:?}''", error);
                })
            } else {
                panic!("can't open the file ''{:?}''", error);
            }
        });
    }
    {
        fn main7() -> Result<(), Box<dyn Error>> {
            let f = File::open("hello.txt")?;

            Ok(())
        }

        main7();
    }
}
